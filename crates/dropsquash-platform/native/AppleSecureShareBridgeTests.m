#import <assert.h>
#import "AppleSecureShareBridge.m"

static uint8_t *receivedBytes;
static size_t receivedLength;

static void receiveFrame(
    int32_t status, uint32_t width, uint32_t height, uint32_t stride,
    uint8_t *bytes, size_t length, void *context
) {
    assert(status == 0);
    assert(width == 2 && height == 2 && stride >= 8);
    assert(context == (void *)1);
    receivedBytes = bytes;
    receivedLength = length;
}

int main(void) {
    CVPixelBufferRef pixel = NULL;
    assert(CVPixelBufferCreate(NULL, 2, 2, kCVPixelFormatType_32BGRA, NULL, &pixel) == kCVReturnSuccess);
    assert(CVPixelBufferLockBaseAddress(pixel, 0) == kCVReturnSuccess);
    memset(CVPixelBufferGetBaseAddress(pixel), 0x7f, CVPixelBufferGetDataSize(pixel));
    CVPixelBufferUnlockBaseAddress(pixel, 0);
    CMVideoFormatDescriptionRef format = NULL;
    assert(CMVideoFormatDescriptionCreateForImageBuffer(NULL, pixel, &format) == noErr);
    CMSampleTimingInfo timing = { kCMTimeZero, kCMTimeZero, kCMTimeInvalid };
    CMSampleBufferRef sample = NULL;
    assert(CMSampleBufferCreateReadyWithImageBuffer(NULL, pixel, format, &timing, &sample) == noErr);
    DSQCopyFrame(sample, YES, (void *)1, receiveFrame);
    assert(receivedBytes != NULL && receivedLength > 0);
    for (size_t index = 0; index < receivedLength; index += 1) assert(receivedBytes[index] == 0);
    dropsquash_secure_share_bridge_release_frame(receivedBytes);
    CFRelease(sample);
    CFRelease(format);
    CVPixelBufferRelease(pixel);
    return 0;
}
