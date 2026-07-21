#import "AppleSecureShareBridge.h"
#import <AVFoundation/AVFoundation.h>
#import <ApplicationServices/ApplicationServices.h>
#import <AppKit/AppKit.h>
#import <CoreVideo/CoreVideo.h>
#import <ScreenCaptureKit/ScreenCaptureKit.h>
#import <Vision/Vision.h>
#import <mach/mach_time.h>
#import <math.h>

@interface DSQStrictRecordingSession : NSObject <SCStreamOutput>
- (instancetype)initWithID:(uint64_t)sessionID
                  windowID:(uint32_t)windowID
                     width:(uint32_t)width
                    height:(uint32_t)height
                      path:(NSString *)path
                  context:(void *)context
                  callback:(DSQRecordingCallback)callback
          metadataCallback:(DSQFrameMetadataCallback)metadataCallback
            visionCallback:(DSQVisionObservationCallback)visionCallback
     accessibilityCallback:(DSQAccessibilityObservationCallback)accessibilityCallback
         temporalCallback:(DSQTemporalObservationCallback)temporalCallback
       destructionCallback:(DSQDestructionEvidenceCallback)destructionCallback;
- (void)start;
- (void)stop;
@end

static NSMutableDictionary<NSNumber *, DSQStrictRecordingSession *> *DSQSessions(void) {
    static NSMutableDictionary<NSNumber *, DSQStrictRecordingSession *> *sessions;
    static dispatch_once_t once;
    dispatch_once(&once, ^{ sessions = [NSMutableDictionary new]; });
    return sessions;
}

static uint64_t DSQNextSessionID = 1;
static const uint64_t DSQVisionSampleInterval = 15;
static void DSQDisplayChanged(CGDirectDisplayID, CGDisplayChangeSummaryFlags, void *);
static void DSQAccessibilityChanged(AXObserverRef, AXUIElementRef, CFStringRef, void *);

static BOOL DSQFaultInjectionMatchesFrame(uint64_t frameIndex) {
    NSDictionary<NSString *, NSString *> *environment = NSProcessInfo.processInfo.environment;
    if (![environment[@"DROP_SQUASH_QA_ENABLE_NATIVE_FAULTS"] isEqualToString:@"1"]) return NO;
    NSString *value = environment[@"DROP_SQUASH_QA_NATIVE_VISION_FAIL_FRAME"];
    if (value.length == 0) return NO;
    NSScanner *scanner = [NSScanner scannerWithString:value];
    unsigned long long expected = 0;
    return [scanner scanUnsignedLongLong:&expected] && scanner.isAtEnd && expected == frameIndex;
}

static BOOL DSQIsTextElement(AXUIElementRef element) {
    CFTypeRef role = NULL;
    if (AXUIElementCopyAttributeValue(element, kAXRoleAttribute, &role) != kAXErrorSuccess || role == NULL) return NO;
    BOOL text = CFEqual(role, kAXStaticTextRole) || CFEqual(role, kAXTextFieldRole) ||
        CFEqual(role, kAXTextAreaRole) || CFEqual(role, kAXComboBoxRole);
    CFRelease(role);
    return text;
}

typedef NS_ENUM(int32_t, DSQRecordingFailure) {
    DSQRecordingFailureAuthorization = 1,
    DSQRecordingFailureTarget = 2,
    DSQRecordingFailureSetup = 3,
    DSQRecordingFailureEnvironment = 4,
    DSQRecordingFailureWriter = 5,
    DSQRecordingFailureContinuity = 6,
};

@interface DSQStrictRecordingSession ()
@property(nonatomic) uint64_t sessionID;
@property(nonatomic) uint32_t windowID;
@property(nonatomic) uint32_t width;
@property(nonatomic) uint32_t height;
@property(nonatomic) void *context;
@property(nonatomic) DSQRecordingCallback callback;
@property(nonatomic) DSQFrameMetadataCallback metadataCallback;
@property(nonatomic) DSQVisionObservationCallback visionCallback;
@property(nonatomic) DSQAccessibilityObservationCallback accessibilityCallback;
@property(nonatomic) DSQTemporalObservationCallback temporalCallback;
@property(nonatomic) DSQDestructionEvidenceCallback destructionCallback;
@property(nonatomic) uint64_t frameCount;
@property(nonatomic) BOOL started;
@property(nonatomic) BOOL stopping;
@property(nonatomic) BOOL failed;
@property(nonatomic) BOOL sessionClockStarted;
@property(nonatomic) BOOL hasMetadata;
@property(nonatomic) uint64_t lastDisplayTime;
@property(nonatomic) DSQFrameMetadata baselineMetadata;
@property(nonatomic) BOOL requiresAttestation;
@property(nonatomic) int32_t expectedOwnerPID;
@property(nonatomic) CGRect expectedFrame;
@property(nonatomic) BOOL displayFailed;
@property(nonatomic) BOOL lifecycleFailed;
@property(nonatomic) BOOL focusFailed;
@property(nonatomic) BOOL focusedWindowMismatch;
@property(nonatomic) BOOL accessibilityEventFailed;
@property(nonatomic) BOOL visionFailed;
@property(nonatomic) BOOL displayWatching;
@property(nonatomic) AXObserverRef accessibilityObserver;
@property(nonatomic) AXUIElementRef accessibilityWindow;
@property(nonatomic, strong) NSMutableArray *workspaceObservers;
@property(nonatomic, strong) NSString *path;
@property(nonatomic, strong) SCStream *stream;
@property(nonatomic, strong) AVAssetWriter *writer;
@property(nonatomic, strong) AVAssetWriterInput *input;
@property(nonatomic, strong) AVAssetWriterInputPixelBufferAdaptor *adaptor;
@property(nonatomic) dispatch_queue_t frameQueue;
@property(nonatomic) dispatch_queue_t visionQueue;
@property(nonatomic) dispatch_group_t visionGroup;
@property(nonatomic) NSUInteger visionInFlight;
@property(nonatomic, strong) NSData *previousFrameBytes;
@end

@implementation DSQStrictRecordingSession

- (instancetype)initWithID:(uint64_t)sessionID
                  windowID:(uint32_t)windowID
                     width:(uint32_t)width
                    height:(uint32_t)height
                      path:(NSString *)path
                  context:(void *)context
                  callback:(DSQRecordingCallback)callback
          metadataCallback:(DSQFrameMetadataCallback)metadataCallback
            visionCallback:(DSQVisionObservationCallback)visionCallback
     accessibilityCallback:(DSQAccessibilityObservationCallback)accessibilityCallback
         temporalCallback:(DSQTemporalObservationCallback)temporalCallback
       destructionCallback:(DSQDestructionEvidenceCallback)destructionCallback {
    self = [super init];
    if (self != nil) {
        _sessionID = sessionID;
        _windowID = windowID;
        _width = width;
        _height = height;
        _path = path;
        _context = context;
        _callback = callback;
        _metadataCallback = metadataCallback;
        _visionCallback = visionCallback;
        _accessibilityCallback = accessibilityCallback;
        _temporalCallback = temporalCallback;
        _destructionCallback = destructionCallback;
        _frameQueue = dispatch_queue_create("app.dropsquash.secure-share.frames", DISPATCH_QUEUE_SERIAL);
        _visionQueue = dispatch_queue_create("app.dropsquash.secure-share.vision", DISPATCH_QUEUE_SERIAL);
        _visionGroup = dispatch_group_create();
    }
    return self;
}

- (BOOL)emitAccessibilityTree:(AXUIElementRef)element frame:(uint64_t)frameIndex {
    if (self.accessibilityCallback == NULL) return YES;
    if (DSQIsTextElement(element) && ![self emitAccessibilityElement:element kind:1 frame:frameIndex]) return NO;
    CFTypeRef rawChildren = NULL;
    AXError error = AXUIElementCopyAttributeValue(element, kAXChildrenAttribute, &rawChildren);
    if (error != kAXErrorSuccess || rawChildren == NULL) return YES;
    if (CFGetTypeID(rawChildren) != CFArrayGetTypeID()) {
        if (rawChildren != NULL) CFRelease(rawChildren);
        return NO;
    }
    CFArrayRef children = rawChildren;
    for (CFIndex index = 0; index < CFArrayGetCount(children); index++) {
        if (![self emitAccessibilityTree:(AXUIElementRef)CFArrayGetValueAtIndex(children, index) frame:frameIndex]) {
            CFRelease(children); return NO;
        }
    }
    CFRelease(children);
    return YES;
}

- (BOOL)emitFocusedTextElement:(uint64_t)frameIndex {
    if (self.accessibilityCallback == NULL) return YES;
    AXUIElementRef application = AXUIElementCreateApplication(self.expectedOwnerPID);
    if (application == NULL) return NO;
    CFTypeRef focused = NULL;
    AXError error = AXUIElementCopyAttributeValue(application, kAXFocusedUIElementAttribute, &focused);
    CFRelease(application);
    if (error != kAXErrorSuccess || focused == NULL) return YES;
    if (CFGetTypeID(focused) != AXUIElementGetTypeID()) { CFRelease(focused); return NO; }
    AXUIElementRef element = focused;
    BOOL emitted = !DSQIsTextElement(element) ||
        ([self focusedElementMatchesSelectedWindow:element] &&
         [self emitAccessibilityElement:element kind:2 frame:frameIndex]);
    CFRelease(focused);
    return emitted;
}

- (BOOL)focusedElementMatchesSelectedWindow:(AXUIElementRef)element {
    if (self.accessibilityWindow == NULL) return NO;
    CFTypeRef window = NULL;
    AXError error = AXUIElementCopyAttributeValue(element, kAXWindowAttribute, &window);
    if (error == kAXErrorSuccess && window != NULL && CFGetTypeID(window) == AXUIElementGetTypeID()) {
        BOOL matches = CFEqual(window, self.accessibilityWindow);
        CFRelease(window);
        if (!matches) self.focusedWindowMismatch = YES;
        return matches;
    }
    if (window != NULL) CFRelease(window);
    AXUIElementRef current = element;
    CFRetain(current);
    NSMutableSet<NSValue *> *visited = [NSMutableSet new];
    while (current != NULL) {
        if (CFEqual(current, self.accessibilityWindow)) { CFRelease(current); return YES; }
        NSValue *identity = [NSValue valueWithPointer:current];
        if ([visited containsObject:identity]) { CFRelease(current); break; }
        [visited addObject:identity];
        CFTypeRef parent = NULL;
        AXError parentError = AXUIElementCopyAttributeValue(current, kAXParentAttribute, &parent);
        CFRelease(current);
        if (parentError != kAXErrorSuccess || parent == NULL || CFGetTypeID(parent) != AXUIElementGetTypeID()) {
            if (parent != NULL) CFRelease(parent);
            break;
        }
        current = (AXUIElementRef)parent;
    }
    self.focusedWindowMismatch = YES;
    return NO;
}

- (BOOL)emitAccessibilityElement:(AXUIElementRef)element kind:(int32_t)kind frame:(uint64_t)frameIndex {
    CFTypeRef position = NULL;
    CFTypeRef size = NULL;
    AXError positionError = AXUIElementCopyAttributeValue(element, kAXPositionAttribute, &position);
    AXError sizeError = AXUIElementCopyAttributeValue(element, kAXSizeAttribute, &size);
    CGPoint origin;
    CGSize dimensions;
    BOOL valid = positionError == kAXErrorSuccess && sizeError == kAXErrorSuccess && position != NULL && size != NULL &&
        AXValueGetType(position) == kAXValueCGPointType && AXValueGetType(size) == kAXValueCGSizeType &&
        AXValueGetValue(position, kAXValueCGPointType, &origin) && AXValueGetValue(size, kAXValueCGSizeType, &dimensions) &&
        isfinite(origin.x) && isfinite(origin.y) && isfinite(dimensions.width) && isfinite(dimensions.height) &&
        dimensions.width > 0 && dimensions.height > 0 && origin.x >= INT32_MIN && origin.x <= INT32_MAX &&
        origin.y >= INT32_MIN && origin.y <= INT32_MAX && dimensions.width <= UINT32_MAX && dimensions.height <= UINT32_MAX;
    if (position != NULL) CFRelease(position);
    if (size != NULL) CFRelease(size);
    if (!valid) return YES;
    DSQAccessibilityObservation observation = { .frameIndex = frameIndex, .kind = kind, .x = (int32_t)llround(origin.x),
        .y = (int32_t)llround(origin.y), .width = (uint32_t)llround(dimensions.width),
        .height = (uint32_t)llround(dimensions.height) };
    return self.accessibilityCallback(self.sessionID, &observation, self.context) == 0;
}

- (BOOL)emitVision:(CVPixelBufferRef)pixel frame:(uint64_t)frameIndex {
    if (self.visionCallback == NULL) return YES;
    VNRecognizeTextRequest *text = [VNRecognizeTextRequest new];
    text.recognitionLanguages = @[ @"ja-JP", @"en-US" ];
    text.usesLanguageCorrection = NO;
    text.recognitionLevel = VNRequestTextRecognitionLevelAccurate;
    VNDetectTextRectanglesRequest *shapes = [VNDetectTextRectanglesRequest new];
    VNImageRequestHandler *handler = [[VNImageRequestHandler alloc] initWithCVPixelBuffer:pixel options:@{}];
    NSError *error = nil;
    if (![handler performRequests:@[ text, shapes ] error:&error]) return NO;
    for (VNRecognizedTextObservation *item in text.results) {
        if (![self emitVisionRect:item.boundingBox confidence:item.confidence kind:1 frame:frameIndex]) return NO;
    }
    for (VNTextObservation *item in shapes.results) {
        if (![self emitVisionRect:item.boundingBox confidence:item.confidence kind:2 frame:frameIndex]) return NO;
    }
    return YES;
}

- (BOOL)scheduleVision:(CVPixelBufferRef)source frame:(uint64_t)frameIndex {
    if (self.visionCallback == NULL) return YES;
    if (CVPixelBufferGetPixelFormatType(source) != kCVPixelFormatType_32BGRA) return NO;
    const size_t width = CVPixelBufferGetWidth(source);
    const size_t height = CVPixelBufferGetHeight(source);
    const size_t bytesPerRow = CVPixelBufferGetBytesPerRow(source);
    if (width == 0 || height == 0 || bytesPerRow < width * 4 || height > SIZE_MAX / bytesPerRow ||
        CVPixelBufferLockBaseAddress(source, kCVPixelBufferLock_ReadOnly) != kCVReturnSuccess) return NO;
    void *base = CVPixelBufferGetBaseAddress(source);
    if (base == NULL) { CVPixelBufferUnlockBaseAddress(source, kCVPixelBufferLock_ReadOnly); return NO; }
    NSData *bytes = [NSData dataWithBytes:base length:bytesPerRow * height];
    CVPixelBufferUnlockBaseAddress(source, kCVPixelBufferLock_ReadOnly);
    if (bytes == nil || bytes.length != bytesPerRow * height) return NO;
    @synchronized (self) {
        if (self.visionInFlight >= 3) return NO;
        self.visionInFlight += 1;
    }
    dispatch_group_enter(self.visionGroup);
    DSQStrictRecordingSession *session = self;
    dispatch_async(self.visionQueue, ^{
        BOOL emitted = [session emitVisionBytes:bytes width:width height:height bytesPerRow:bytesPerRow frame:frameIndex];
        @synchronized (session) {
            session.visionInFlight -= 1;
            session.visionFailed = session.visionFailed || !emitted;
        }
        dispatch_group_leave(session.visionGroup);
    });
    return YES;
}

- (BOOL)emitVisionBytes:(NSData *)bytes width:(size_t)width height:(size_t)height bytesPerRow:(size_t)bytesPerRow frame:(uint64_t)frameIndex {
    CVPixelBufferRef pixel = NULL;
    NSDictionary *attributes = @{ (id)kCVPixelBufferPixelFormatTypeKey: @(kCVPixelFormatType_32BGRA) };
    if (CVPixelBufferCreate(NULL, width, height, kCVPixelFormatType_32BGRA, (__bridge CFDictionaryRef)attributes, &pixel) != kCVReturnSuccess) return NO;
    if (CVPixelBufferLockBaseAddress(pixel, 0) != kCVReturnSuccess) { CVPixelBufferRelease(pixel); return NO; }
    const size_t destinationRow = CVPixelBufferGetBytesPerRow(pixel);
    if (destinationRow < width * 4) { CVPixelBufferUnlockBaseAddress(pixel, 0); CVPixelBufferRelease(pixel); return NO; }
    uint8_t *destination = CVPixelBufferGetBaseAddress(pixel);
    const uint8_t *source = bytes.bytes;
    for (size_t row = 0; row < height; row++) memcpy(destination + row * destinationRow, source + row * bytesPerRow, width * 4);
    CVPixelBufferUnlockBaseAddress(pixel, 0);
    BOOL emitted = [self emitVision:pixel frame:frameIndex];
    CVPixelBufferRelease(pixel);
    return emitted;
}

- (BOOL)emitVisionRect:(CGRect)rect confidence:(float)confidence kind:(int32_t)kind frame:(uint64_t)frameIndex {
    if (!isfinite(rect.origin.x) || !isfinite(rect.origin.y) || !isfinite(rect.size.width) ||
        !isfinite(rect.size.height) || !isfinite(confidence) || rect.size.width <= 0 || rect.size.height <= 0) return NO;
    DSQVisionObservation observation = { .frameIndex = frameIndex, .kind = kind,
        .x = rect.origin.x, .y = rect.origin.y, .width = rect.size.width,
        .height = rect.size.height, .confidence = confidence };
    return self.visionCallback(self.sessionID, &observation, self.context) == 0;
}

- (NSData *)copyFramePixels:(CVPixelBufferRef)source {
    size_t packedRow = (size_t)self.width * 4;
    if (CVPixelBufferGetPixelFormatType(source) != kCVPixelFormatType_32BGRA ||
        CVPixelBufferGetWidth(source) != self.width || CVPixelBufferGetHeight(source) != self.height ||
        packedRow == 0 || self.height > SIZE_MAX / packedRow ||
        CVPixelBufferLockBaseAddress(source, kCVPixelBufferLock_ReadOnly) != kCVReturnSuccess) return nil;
    size_t row = CVPixelBufferGetBytesPerRow(source);
    const uint8_t *base = CVPixelBufferGetBaseAddress(source);
    NSMutableData *copy = (base == NULL || row < packedRow) ? nil : [NSMutableData dataWithLength:packedRow * self.height];
    if (copy != nil) for (size_t y = 0; y < self.height; y++) {
        memcpy((uint8_t *)copy.mutableBytes + y * packedRow, base + y * row, packedRow);
    }
    CVPixelBufferUnlockBaseAddress(source, kCVPixelBufferLock_ReadOnly);
    return copy;
}

- (BOOL)emitTemporalChange:(CVPixelBufferRef)source frame:(uint64_t)frameIndex {
    if (self.temporalCallback == NULL) return YES;
    NSData *current = [self copyFramePixels:source];
    if (current == nil) return NO;
    NSData *previous = self.previousFrameBytes;
    self.previousFrameBytes = current;
    if (previous == nil) return YES;
    if (previous.length != current.length) return NO;
    const uint8_t *before = previous.bytes;
    const uint8_t *after = current.bytes;
    uint32_t left = self.width, top = self.height, right = 0, bottom = 0;
    for (uint32_t y = 0; y < self.height; y++) for (uint32_t x = 0; x < self.width; x++) {
        size_t offset = ((size_t)y * self.width + x) * 4;
        if (memcmp(before + offset, after + offset, 4) == 0) continue;
        left = MIN(left, x); top = MIN(top, y); right = MAX(right, x + 1); bottom = MAX(bottom, y + 1);
    }
    if (right == 0 || bottom == 0) return YES;
    DSQTemporalObservation observation = { .frameIndex = frameIndex, .x = (float)left / self.width,
        .y = (float)top / self.height, .width = (float)(right - left) / self.width,
        .height = (float)(bottom - top) / self.height };
    return self.temporalCallback(self.sessionID, &observation, self.context) == 0;
}

- (BOOL)readRect:(id)value x:(int32_t *)x y:(int32_t *)y width:(uint32_t *)width height:(uint32_t *)height {
    CGRect rect;
    if ([value isKindOfClass:[NSValue class]]) rect = [value CGRectValue];
    else if ([value isKindOfClass:[NSDictionary class]]) {
        NSDictionary *dict = value;
        rect = CGRectMake([dict[@"X"] doubleValue], [dict[@"Y"] doubleValue],
                          [dict[@"Width"] doubleValue], [dict[@"Height"] doubleValue]);
    } else return NO;
    if (!isfinite(rect.origin.x) || !isfinite(rect.origin.y) || !isfinite(rect.size.width) ||
        !isfinite(rect.size.height) || rect.origin.x < INT32_MIN || rect.origin.x > INT32_MAX ||
        rect.origin.y < INT32_MIN || rect.origin.y > INT32_MAX || rect.size.width <= 0 ||
        rect.size.height <= 0 || rect.size.width > UINT32_MAX || rect.size.height > UINT32_MAX) return NO;
    *x = (int32_t)llround(rect.origin.x); *y = (int32_t)llround(rect.origin.y);
    *width = (uint32_t)llround(rect.size.width); *height = (uint32_t)llround(rect.size.height);
    return *width > 0 && *height > 0;
}

- (BOOL)emitMetadata:(CMSampleBufferRef)sample {
    CFArrayRef attachments = CMSampleBufferGetSampleAttachmentsArray(sample, false);
    NSDictionary *info = attachments == NULL ? nil : ((__bridge NSArray *)attachments).firstObject;
    NSNumber *status = info[SCStreamFrameInfoStatus];
    NSNumber *time = info[SCStreamFrameInfoDisplayTime];
    NSNumber *scale = info[SCStreamFrameInfoScaleFactor];
    NSNumber *contentScale = info[SCStreamFrameInfoContentScale];
    if (status == nil || time == nil || scale == nil || contentScale == nil ||
        !isfinite(scale.floatValue) || !isfinite(contentScale.floatValue) ||
        scale.floatValue <= 0 || contentScale.floatValue <= 0 || status.integerValue < 0 || status.integerValue > 5) return NO;
    DSQFrameMetadata metadata = { .frameIndex = self.frameCount,
        .displayTimeTicks = time.unsignedLongLongValue, .frameStatus = (int32_t)status.integerValue,
        .scaleFactor = scale.floatValue, .contentScale = contentScale.floatValue };
    if (![self readRect:info[SCStreamFrameInfoContentRect] x:&metadata.contentX y:&metadata.contentY
                  width:&metadata.contentWidth height:&metadata.contentHeight] ||
        ![self readRect:info[SCStreamFrameInfoBoundingRect] x:&metadata.boundingX y:&metadata.boundingY
                  width:&metadata.boundingWidth height:&metadata.boundingHeight]) return NO;
    if (![self acceptMetadata:metadata]) return NO;
    if (self.metadataCallback == NULL) return YES;
    return self.metadataCallback(self.sessionID, &metadata, self.context) == 0;
}

- (BOOL)acceptMetadata:(DSQFrameMetadata)metadata {
    if (metadata.frameStatus != 0) return NO;
    if (self.hasMetadata && (metadata.displayTimeTicks <= self.lastDisplayTime ||
        [self gapTooLargeFrom:self.lastDisplayTime to:metadata.displayTimeTicks] ||
        ![self matchesBaseline:metadata])) return NO;
    if (!self.hasMetadata) self.baselineMetadata = metadata;
    self.hasMetadata = YES;
    self.lastDisplayTime = metadata.displayTimeTicks;
    return YES;
}

- (BOOL)gapTooLargeFrom:(uint64_t)previous to:(uint64_t)current {
    mach_timebase_info_data_t scale;
    if (mach_timebase_info(&scale) != KERN_SUCCESS || scale.denom == 0) return YES;
    __uint128_t nanoseconds = (__uint128_t)(current - previous) * scale.numer / scale.denom;
    return nanoseconds > 166666665;
}

- (BOOL)matchesBaseline:(DSQFrameMetadata)value {
    DSQFrameMetadata baseline = self.baselineMetadata;
    return value.scaleFactor == baseline.scaleFactor && value.contentScale == baseline.contentScale &&
        value.contentX == baseline.contentX && value.contentY == baseline.contentY &&
        value.contentWidth == baseline.contentWidth && value.contentHeight == baseline.contentHeight &&
        value.boundingX == baseline.boundingX && value.boundingY == baseline.boundingY &&
        value.boundingWidth == baseline.boundingWidth && value.boundingHeight == baseline.boundingHeight;
}

- (BOOL)startWatchers {
    if (!self.requiresAttestation) return YES;
    self.workspaceObservers = [NSMutableArray new];
    NSNotificationCenter *center = NSWorkspace.sharedWorkspace.notificationCenter;
    __weak DSQStrictRecordingSession *weakSelf = self;
    NSArray<NSNotificationName> *lifecycle = @[
        NSWorkspaceSessionDidResignActiveNotification, NSWorkspaceWillSleepNotification,
        NSWorkspaceScreensDidSleepNotification, NSWorkspaceWillPowerOffNotification,
        NSWorkspaceActiveSpaceDidChangeNotification
    ];
    for (NSNotificationName name in lifecycle) {
        id observer = [center addObserverForName:name object:nil queue:nil usingBlock:^(NSNotification *note) {
            (void)note;
            DSQStrictRecordingSession *session = weakSelf;
            if (session != nil) @synchronized (session) { session.lifecycleFailed = YES; }
        }];
        if (observer == nil) return NO;
        [self.workspaceObservers addObject:observer];
    }
    id activation = [center addObserverForName:NSWorkspaceDidActivateApplicationNotification object:nil queue:nil
                                    usingBlock:^(NSNotification *note) {
        DSQStrictRecordingSession *session = weakSelf;
        NSRunningApplication *app = note.userInfo[NSWorkspaceApplicationKey];
        if (session != nil && app != nil && app.processIdentifier != session.expectedOwnerPID &&
            app.processIdentifier != getpid()) @synchronized (session) { session.focusFailed = YES; }
    }];
    if (activation == nil) return NO;
    [self.workspaceObservers addObject:activation];
    if (CGDisplayRegisterReconfigurationCallback(DSQDisplayChanged, (__bridge void *)self) != kCGErrorSuccess) return NO;
    self.displayWatching = YES;
    return [self startAccessibilityWatcher];
}

- (void)stopWatchers {
    if (self.displayWatching) {
        CGDisplayRemoveReconfigurationCallback(DSQDisplayChanged, (__bridge void *)self);
        self.displayWatching = NO;
    }
    NSNotificationCenter *center = NSWorkspace.sharedWorkspace.notificationCenter;
    for (id observer in self.workspaceObservers) [center removeObserver:observer];
    self.workspaceObservers = nil;
    if (self.accessibilityObserver != NULL && self.accessibilityWindow != NULL) {
        AXObserverRemoveNotification(self.accessibilityObserver, self.accessibilityWindow, kAXMovedNotification);
        AXObserverRemoveNotification(self.accessibilityObserver, self.accessibilityWindow, kAXResizedNotification);
        AXObserverRemoveNotification(self.accessibilityObserver, self.accessibilityWindow, kAXUIElementDestroyedNotification);
        CFRunLoopRemoveSource(CFRunLoopGetMain(), AXObserverGetRunLoopSource(self.accessibilityObserver), kCFRunLoopCommonModes);
        CFRelease(self.accessibilityWindow);
        CFRelease(self.accessibilityObserver);
        self.accessibilityWindow = NULL;
        self.accessibilityObserver = NULL;
    }
}

- (BOOL)watchFailed {
    return self.displayFailed || self.lifecycleFailed || self.focusFailed || self.accessibilityEventFailed;
}

- (BOOL)attestWindow {
    if (!self.requiresAttestation) return YES;
    CFArrayRef copied = CGWindowListCopyWindowInfo(
        kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements, kCGNullWindowID);
    NSArray<NSDictionary *> *windows = CFBridgingRelease(copied);
    for (NSDictionary *window in windows) {
        NSNumber *number = window[(__bridge NSString *)kCGWindowNumber];
        if (number.unsignedIntValue != self.windowID) continue;
        NSNumber *owner = window[(__bridge NSString *)kCGWindowOwnerPID];
        NSDictionary *bounds = window[(__bridge NSString *)kCGWindowBounds];
        CGRect frame;
        return owner.intValue == self.expectedOwnerPID && bounds != nil &&
            CGRectMakeWithDictionaryRepresentation((__bridge CFDictionaryRef)bounds, &frame) &&
            CGRectEqualToRect(frame, self.expectedFrame);
    }
    return NO;
}

- (BOOL)attestAccessibilityWindow {
    if (!self.requiresAttestation) return YES;
    AXUIElementRef window = [self matchingAccessibilityWindow];
    if (window == NULL) return NO;
    CFRelease(window);
    return YES;
}

- (AXUIElementRef)matchingAccessibilityWindow {
    AXUIElementRef app = AXUIElementCreateApplication(self.expectedOwnerPID);
    CFTypeRef rawWindows = NULL;
    AXError error = AXUIElementCopyAttributeValue(app, kAXWindowsAttribute, &rawWindows);
    CFRelease(app);
    if (error != kAXErrorSuccess || rawWindows == NULL || CFGetTypeID(rawWindows) != CFArrayGetTypeID()) {
        if (rawWindows != NULL) CFRelease(rawWindows);
        return NULL;
    }
    CFArrayRef windows = rawWindows;
    for (CFIndex index = 0; index < CFArrayGetCount(windows); index++) {
        AXUIElementRef window = (AXUIElementRef)CFArrayGetValueAtIndex(windows, index);
        if ([self accessibilityWindow:window matchesFrame:self.expectedFrame]) {
            CFRetain(window);
            CFRelease(windows);
            return window;
        }
    }
    CFRelease(windows);
    return NULL;
}

- (BOOL)startAccessibilityWatcher {
    if (!self.requiresAttestation) return YES;
    AXUIElementRef window = [self matchingAccessibilityWindow];
    AXObserverRef observer = NULL;
    if (window == NULL || AXObserverCreate(self.expectedOwnerPID, DSQAccessibilityChanged, &observer) != kAXErrorSuccess) {
        if (window != NULL) CFRelease(window);
        return NO;
    }
    void *context = (__bridge void *)self;
    if (AXObserverAddNotification(observer, window, kAXMovedNotification, context) != kAXErrorSuccess ||
        AXObserverAddNotification(observer, window, kAXResizedNotification, context) != kAXErrorSuccess ||
        AXObserverAddNotification(observer, window, kAXUIElementDestroyedNotification, context) != kAXErrorSuccess) {
        CFRelease(window);
        CFRelease(observer);
        return NO;
    }
    CFRunLoopAddSource(CFRunLoopGetMain(), AXObserverGetRunLoopSource(observer), kCFRunLoopCommonModes);
    self.accessibilityWindow = window;
    self.accessibilityObserver = observer;
    return YES;
}

- (BOOL)accessibilityWindow:(AXUIElementRef)window matchesFrame:(CGRect)expected {
    CFTypeRef positionValue = NULL;
    CFTypeRef sizeValue = NULL;
    AXError position = AXUIElementCopyAttributeValue(window, kAXPositionAttribute, &positionValue);
    AXError size = AXUIElementCopyAttributeValue(window, kAXSizeAttribute, &sizeValue);
    CGPoint origin;
    CGSize dimensions;
    BOOL matches = position == kAXErrorSuccess && size == kAXErrorSuccess && positionValue != NULL && sizeValue != NULL &&
        AXValueGetType(positionValue) == kAXValueCGPointType && AXValueGetType(sizeValue) == kAXValueCGSizeType &&
        AXValueGetValue(positionValue, kAXValueCGPointType, &origin) &&
        AXValueGetValue(sizeValue, kAXValueCGSizeType, &dimensions) &&
        CGRectEqualToRect(CGRectMake(origin.x, origin.y, dimensions.width, dimensions.height), expected);
    if (positionValue != NULL) CFRelease(positionValue);
    if (sizeValue != NULL) CFRelease(sizeValue);
    return matches;
}

- (void)start {
    if (self.requiresAttestation && !AXIsProcessTrusted()) { [self fail:DSQRecordingFailureAuthorization]; return; }
    [SCShareableContent getShareableContentExcludingDesktopWindows:YES
                                               onScreenWindowsOnly:YES
                                                 completionHandler:^(SCShareableContent *content, NSError *error) {
        SCWindow *window = nil;
        for (SCWindow *candidate in content.windows) {
            if (candidate.windowID == self.windowID) { window = candidate; break; }
        }
        if (error != nil || window == nil || ![self attestWindow] || ![self attestAccessibilityWindow]) { [self fail:DSQRecordingFailureTarget]; return; }
        if (![self prepareWriter]) { [self fail:DSQRecordingFailureWriter]; return; }
        SCContentFilter *filter = [[SCContentFilter alloc] initWithDesktopIndependentWindow:window];
        SCStreamConfiguration *configuration = [SCStreamConfiguration new];
        configuration.width = self.width;
        configuration.height = self.height;
        configuration.pixelFormat = kCVPixelFormatType_32BGRA;
        configuration.minimumFrameInterval = CMTimeMake(1, 30);
        configuration.capturesAudio = NO;
        configuration.showsCursor = NO;
        self.stream = [[SCStream alloc] initWithFilter:filter configuration:configuration delegate:nil];
        NSError *addError = nil;
        if (![self.stream addStreamOutput:self type:SCStreamOutputTypeScreen
                       sampleHandlerQueue:self.frameQueue error:&addError]) { [self fail:DSQRecordingFailureSetup]; return; }
        if (![self startWatchers]) { [self fail:DSQRecordingFailureSetup]; return; }
        [self.stream startCaptureWithCompletionHandler:^(NSError *startError) {
            if (startError != nil) { [self fail:DSQRecordingFailureSetup]; return; }
            @synchronized (self) { self.started = YES; }
            self.callback(1, 0, self.sessionID, 0, self.context);
            @synchronized (self) { if (self.stopping) [self stop]; }
        }];
    }];
}

- (BOOL)prepareWriter {
    NSURL *url = [NSURL fileURLWithPath:self.path];
    [[NSFileManager defaultManager] removeItemAtURL:url error:nil];
    NSError *error = nil;
    self.writer = [[AVAssetWriter alloc] initWithURL:url fileType:AVFileTypeMPEG4 error:&error];
    if (error != nil || self.writer == nil) return NO;
    NSDictionary *settings = @{ AVVideoCodecKey: AVVideoCodecTypeH264,
        AVVideoWidthKey: @(self.width), AVVideoHeightKey: @(self.height) };
    self.input = [AVAssetWriterInput assetWriterInputWithMediaType:AVMediaTypeVideo outputSettings:settings];
    self.input.expectsMediaDataInRealTime = YES;
    if (![self.writer canAddInput:self.input]) return NO;
    [self.writer addInput:self.input];
    NSDictionary *attributes = @{ (id)kCVPixelBufferPixelFormatTypeKey: @(kCVPixelFormatType_32BGRA),
        (id)kCVPixelBufferWidthKey: @(self.width), (id)kCVPixelBufferHeightKey: @(self.height) };
    self.adaptor = [[AVAssetWriterInputPixelBufferAdaptor alloc] initWithAssetWriterInput:self.input
                                                                  sourcePixelBufferAttributes:attributes];
    return [self.writer startWriting];
}

- (void)stream:(SCStream *)stream didOutputSampleBuffer:(CMSampleBufferRef)sample ofType:(SCStreamOutputType)type {
    if (type != SCStreamOutputTypeScreen) return;
    @synchronized (self) {
        if (self.stopping || self.failed) return;
        if ([self watchFailed]) { [self fail:DSQRecordingFailureEnvironment]; return; }
        if (![self attestWindow] || ![self attestAccessibilityWindow]) { [self fail:DSQRecordingFailureTarget]; return; }
        if (!self.input.readyForMoreMediaData) { [self fail:DSQRecordingFailureWriter]; return; }
        CMTime time = CMSampleBufferGetPresentationTimeStamp(sample);
        if (!CMTIME_IS_VALID(time)) { [self fail:DSQRecordingFailureContinuity]; return; }
        if (!self.sessionClockStarted) {
            [self.writer startSessionAtSourceTime:time];
            self.sessionClockStarted = YES;
        }
        if (![self emitMetadata:sample]) { [self fail:DSQRecordingFailureContinuity]; return; }
        __block BOOL focusedEmitted = NO;
        dispatch_sync(dispatch_get_main_queue(), ^{
            focusedEmitted = [self emitFocusedTextElement:self.frameCount];
        });
        if (!focusedEmitted) {
            [self fail:self.focusedWindowMismatch ? DSQRecordingFailureTarget : DSQRecordingFailureContinuity];
            return;
        }
        if (self.frameCount % 15 == 0) {
            __block BOOL accessibilityEmitted = NO;
            dispatch_sync(dispatch_get_main_queue(), ^{
                accessibilityEmitted = [self emitAccessibilityTree:self.accessibilityWindow frame:self.frameCount];
            });
            if (!accessibilityEmitted) { [self fail:DSQRecordingFailureContinuity]; return; }
        }
        CVPixelBufferRef pixel = NULL;
        if (CVPixelBufferPoolCreatePixelBuffer(NULL, self.adaptor.pixelBufferPool, &pixel) != kCVReturnSuccess) {
            [self fail:DSQRecordingFailureWriter]; return;
        }
        CVPixelBufferRef source = CMSampleBufferGetImageBuffer(sample);
        if (source == NULL || self.visionFailed || DSQFaultInjectionMatchesFrame(self.frameCount) ||
            (self.frameCount % DSQVisionSampleInterval == 0 && ![self scheduleVision:source frame:self.frameCount])) {
            CVPixelBufferRelease(pixel); [self fail:DSQRecordingFailureContinuity]; return;
        }
        if (![self emitTemporalChange:source frame:self.frameCount]) {
            CVPixelBufferRelease(pixel); [self fail:DSQRecordingFailureContinuity]; return;
        }
        CVReturn locked = CVPixelBufferLockBaseAddress(pixel, 0);
        if (locked != kCVReturnSuccess || CVPixelBufferGetBaseAddress(pixel) == NULL) {
            if (locked == kCVReturnSuccess) CVPixelBufferUnlockBaseAddress(pixel, 0);
            CVPixelBufferRelease(pixel); [self fail:DSQRecordingFailureWriter]; return;
        }
        memset(CVPixelBufferGetBaseAddress(pixel), 0, CVPixelBufferGetDataSize(pixel));
        CVPixelBufferUnlockBaseAddress(pixel, 0);
        BOOL appended = [self.adaptor appendPixelBuffer:pixel withPresentationTime:time];
        CVPixelBufferRelease(pixel);
        if (!appended) { [self fail:DSQRecordingFailureWriter]; return; }
        if (self.destructionCallback != NULL) {
            DSQDestructionEvidence evidence = { .frameIndex = self.frameCount, .policy = 1,
                .regionCount = 1, .outputWidth = self.width, .outputHeight = self.height };
            if (self.destructionCallback(self.sessionID, &evidence, self.context) != 0) {
                [self fail:DSQRecordingFailureContinuity]; return;
            }
        }
        self.frameCount += 1;
    }
}

- (void)stop {
    @synchronized (self) {
        self.stopping = YES;
        if (!self.started || self.stream == nil) return;
    }
    [self.stream stopCaptureWithCompletionHandler:^(NSError *stopError) {
        if (stopError != nil || self.failed || !self.sessionClockStarted) { [self fail:DSQRecordingFailureWriter]; return; }
        dispatch_group_notify(self.visionGroup, self.frameQueue, ^{ [self finishStoppedRecording]; });
    }];
}

- (void)finishStoppedRecording {
    BOOL visionFailed = NO;
    @synchronized (self) {
        if (self.failed) return;
        visionFailed = self.visionFailed;
    }
    if (visionFailed) { [self fail:DSQRecordingFailureContinuity]; return; }
    if (![self attestWindow] || ![self attestAccessibilityWindow]) { [self fail:DSQRecordingFailureTarget]; return; }
    [self.input markAsFinished];
    [self.writer finishWritingWithCompletionHandler:^{
        [self stopWatchers];
        if (self.writer.status != AVAssetWriterStatusCompleted) { [self fail:DSQRecordingFailureWriter]; return; }
        if ([self watchFailed]) { [self fail:DSQRecordingFailureEnvironment]; return; }
        self.callback(2, 0, self.sessionID, self.frameCount, self.context);
        [self remove];
    }];
}

- (void)fail:(DSQRecordingFailure)reason {
    @synchronized (self) { if (self.failed) return; self.failed = YES; }
    [self.writer cancelWriting];
    [self stopWatchers];
    [[NSFileManager defaultManager] removeItemAtPath:self.path error:nil];
    dispatch_group_notify(self.visionGroup, self.frameQueue, ^{
        self.callback(3, reason, self.sessionID, self.frameCount, self.context);
        [self remove];
    });
}

- (void)remove { @synchronized (DSQSessions()) { [DSQSessions() removeObjectForKey:@(self.sessionID)]; } }
@end

static void DSQDisplayChanged(CGDirectDisplayID display, CGDisplayChangeSummaryFlags flags, void *context) {
    (void)display;
    (void)flags;
    DSQStrictRecordingSession *session = (__bridge DSQStrictRecordingSession *)context;
    if (session != nil) @synchronized (session) { session.displayFailed = YES; }
}

static void DSQAccessibilityChanged(AXObserverRef observer, AXUIElementRef element, CFStringRef notification, void *context) {
    (void)observer;
    (void)element;
    (void)notification;
    DSQStrictRecordingSession *session = (__bridge DSQStrictRecordingSession *)context;
    if (session != nil) @synchronized (session) { session.accessibilityEventFailed = YES; }
}

uint64_t dropsquash_secure_share_bridge_start_strict_recording(
    uint32_t windowID, uint32_t width, uint32_t height, const char *path, void *context, DSQRecordingCallback callback
) {
    return dropsquash_secure_share_bridge_start_strict_recording_with_metadata(
        windowID, width, height, path, context, callback, NULL
    );
}

uint64_t dropsquash_secure_share_bridge_start_strict_recording_with_metadata(
    uint32_t windowID, uint32_t width, uint32_t height, const char *path, void *context,
    DSQRecordingCallback callback, DSQFrameMetadataCallback metadataCallback
) {
    if (callback == NULL || path == NULL || width == 0 || height == 0) return 0;
    uint64_t sessionID = __sync_fetch_and_add(&DSQNextSessionID, 1);
    DSQStrictRecordingSession *session = [[DSQStrictRecordingSession alloc]
        initWithID:sessionID windowID:windowID width:width height:height
        path:[NSString stringWithUTF8String:path] context:context callback:callback metadataCallback:metadataCallback visionCallback:NULL accessibilityCallback:NULL temporalCallback:NULL destructionCallback:NULL];
    @synchronized (DSQSessions()) { DSQSessions()[@(sessionID)] = session; }
    [session start];
    return sessionID;
}

void dropsquash_secure_share_bridge_stop_recording(uint64_t sessionID) {
    DSQStrictRecordingSession *session;
    @synchronized (DSQSessions()) { session = DSQSessions()[@(sessionID)]; }
    [session stop];
}

uint64_t dropsquash_secure_share_bridge_start_attested_strict_recording(
    uint32_t windowID, int32_t ownerPID, int32_t x, int32_t y, uint32_t windowWidth, uint32_t windowHeight,
    uint32_t outputWidth, uint32_t outputHeight, const char *path, void *context,
    DSQRecordingCallback callback, DSQFrameMetadataCallback metadataCallback
) {
    return dropsquash_secure_share_bridge_start_attested_strict_recording_with_observations(
        windowID, ownerPID, x, y, windowWidth, windowHeight, outputWidth, outputHeight, path, context,
        callback, metadataCallback, NULL
    );
}

uint64_t dropsquash_secure_share_bridge_start_attested_strict_recording_with_observations(
    uint32_t windowID, int32_t ownerPID, int32_t x, int32_t y, uint32_t windowWidth, uint32_t windowHeight,
    uint32_t outputWidth, uint32_t outputHeight, const char *path, void *context,
    DSQRecordingCallback callback, DSQFrameMetadataCallback metadataCallback, DSQVisionObservationCallback visionCallback
) {
    return dropsquash_secure_share_bridge_start_attested_strict_recording_with_all_observations(
        windowID, ownerPID, x, y, windowWidth, windowHeight, outputWidth, outputHeight, path, context,
        callback, metadataCallback, visionCallback, NULL
    );
}

uint64_t dropsquash_secure_share_bridge_start_attested_strict_recording_with_all_observations(
    uint32_t windowID, int32_t ownerPID, int32_t x, int32_t y, uint32_t windowWidth, uint32_t windowHeight,
    uint32_t outputWidth, uint32_t outputHeight, const char *path, void *context,
    DSQRecordingCallback callback, DSQFrameMetadataCallback metadataCallback, DSQVisionObservationCallback visionCallback,
    DSQAccessibilityObservationCallback accessibilityCallback
) {
    if (ownerPID <= 0 || windowWidth == 0 || windowHeight == 0) return 0;
    if (callback == NULL || path == NULL || outputWidth == 0 || outputHeight == 0) return 0;
    uint64_t sessionID = __sync_fetch_and_add(&DSQNextSessionID, 1);
    DSQStrictRecordingSession *session = [[DSQStrictRecordingSession alloc]
        initWithID:sessionID windowID:windowID width:outputWidth height:outputHeight
        path:[NSString stringWithUTF8String:path] context:context callback:callback metadataCallback:metadataCallback visionCallback:visionCallback accessibilityCallback:accessibilityCallback temporalCallback:NULL destructionCallback:NULL];
    session.requiresAttestation = YES;
    session.expectedOwnerPID = ownerPID;
    session.expectedFrame = CGRectMake(x, y, windowWidth, windowHeight);
    @synchronized (DSQSessions()) {
        DSQSessions()[@(sessionID)] = session;
    }
    [session start];
    return sessionID;
}

uint64_t dropsquash_secure_share_bridge_start_attested_strict_recording_with_temporal_observations(
    uint32_t windowID, int32_t ownerPID, int32_t x, int32_t y, uint32_t windowWidth, uint32_t windowHeight,
    uint32_t outputWidth, uint32_t outputHeight, const char *path, void *context,
    DSQRecordingCallback callback, DSQFrameMetadataCallback metadataCallback, DSQVisionObservationCallback visionCallback,
    DSQAccessibilityObservationCallback accessibilityCallback, DSQTemporalObservationCallback temporalCallback,
    DSQDestructionEvidenceCallback destructionCallback
) {
    if (ownerPID <= 0 || windowWidth == 0 || windowHeight == 0) return 0;
    if (callback == NULL || path == NULL || outputWidth == 0 || outputHeight == 0) return 0;
    uint64_t sessionID = __sync_fetch_and_add(&DSQNextSessionID, 1);
    DSQStrictRecordingSession *session = [[DSQStrictRecordingSession alloc]
        initWithID:sessionID windowID:windowID width:outputWidth height:outputHeight
        path:[NSString stringWithUTF8String:path] context:context callback:callback metadataCallback:metadataCallback visionCallback:visionCallback accessibilityCallback:accessibilityCallback temporalCallback:temporalCallback destructionCallback:destructionCallback];
    session.requiresAttestation = YES;
    session.expectedOwnerPID = ownerPID;
    session.expectedFrame = CGRectMake(x, y, windowWidth, windowHeight);
    @synchronized (DSQSessions()) { DSQSessions()[@(sessionID)] = session; }
    [session start];
    return sessionID;
}
