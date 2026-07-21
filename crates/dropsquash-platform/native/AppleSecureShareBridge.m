#import "AppleSecureShareBridge.h"
#import <Foundation/Foundation.h>
#import <ScreenCaptureKit/ScreenCaptureKit.h>
#import <CoreMedia/CoreMedia.h>
#import <CoreVideo/CoreVideo.h>
#import <stdint.h>

int32_t dropsquash_secure_share_bridge_abi_version(void) {
    return 1;
}

void dropsquash_secure_share_bridge_discover(void *context, DSQDiscoveryCallback callback) {
    if (callback == NULL) {
        return;
    }
    [SCShareableContent getShareableContentExcludingDesktopWindows:YES
                                               onScreenWindowsOnly:YES
                                                 completionHandler:^(SCShareableContent *content, NSError *error) {
        if (error != nil || content == nil) {
            callback(1, 0, 0, context);
            return;
        }
        if (content.windows.count > INT32_MAX || content.applications.count > INT32_MAX) {
            callback(2, 0, 0, context);
            return;
        }
        callback(0, (int32_t)content.windows.count, (int32_t)content.applications.count, context);
    }];
}

void dropsquash_secure_share_bridge_release_frame(uint8_t *bytes) {
    free(bytes);
}

static void DSQCopyFrame(CMSampleBufferRef sample, BOOL strictShield, void *context, DSQFrameCallback callback) {
    CVPixelBufferRef pixelBuffer = CMSampleBufferGetImageBuffer(sample);
    if (pixelBuffer == nil || CVPixelBufferLockBaseAddress(pixelBuffer, kCVPixelBufferLock_ReadOnly) != kCVReturnSuccess) {
        callback(4, 0, 0, 0, NULL, 0, context);
        return;
    }
    size_t height = CVPixelBufferGetHeight(pixelBuffer);
    size_t stride = CVPixelBufferGetBytesPerRow(pixelBuffer);
    size_t width = CVPixelBufferGetWidth(pixelBuffer);
    if (width > UINT32_MAX || height > UINT32_MAX || stride > UINT32_MAX ||
        (height != 0 && stride > SIZE_MAX / height)) {
        CVPixelBufferUnlockBaseAddress(pixelBuffer, kCVPixelBufferLock_ReadOnly);
        callback(6, 0, 0, 0, NULL, 0, context);
        return;
    }
    size_t length = height * stride;
    uint8_t *copy = malloc(length);
    if (copy == NULL) {
        CVPixelBufferUnlockBaseAddress(pixelBuffer, kCVPixelBufferLock_ReadOnly);
        callback(5, 0, 0, 0, NULL, 0, context);
        return;
    }
    memcpy(copy, CVPixelBufferGetBaseAddress(pixelBuffer), length);
    CVPixelBufferUnlockBaseAddress(pixelBuffer, kCVPixelBufferLock_ReadOnly);
    if (strictShield) {
        memset(copy, 0, length);
    }
    callback(0, (uint32_t)width, (uint32_t)height,
             (uint32_t)stride, copy, length, context);
}

void dropsquash_secure_share_bridge_capture_window_frame(
    uint32_t windowID, int32_t strictShield, void *context, DSQFrameCallback callback
) {
    if (callback == NULL) {
        return;
    }
    [SCShareableContent getShareableContentExcludingDesktopWindows:YES
                                               onScreenWindowsOnly:YES
                                                 completionHandler:^(SCShareableContent *content, NSError *error) {
        SCWindow *window = nil;
        for (SCWindow *candidate in content.windows) {
            if (candidate.windowID == windowID) {
                window = candidate;
                break;
            }
        }
        if (error != nil || window == nil) {
            callback(3, 0, 0, 0, NULL, 0, context);
            return;
        }
        SCContentFilter *filter = [[SCContentFilter alloc] initWithDesktopIndependentWindow:window];
        SCStreamConfiguration *configuration = [SCStreamConfiguration new];
        configuration.width = MAX(1, (size_t)window.frame.size.width);
        configuration.height = MAX(1, (size_t)window.frame.size.height);
        configuration.pixelFormat = kCVPixelFormatType_32BGRA;
        configuration.capturesAudio = NO;
        configuration.showsCursor = NO;
        [SCScreenshotManager captureSampleBufferWithFilter:filter configuration:configuration
                                          completionHandler:^(CMSampleBufferRef sample, NSError *captureError) {
            if (captureError != nil || sample == nil) {
                callback(3, 0, 0, 0, NULL, 0, context);
                return;
            }
            DSQCopyFrame(sample, strictShield != 0, context, callback);
        }];
    }];
}
