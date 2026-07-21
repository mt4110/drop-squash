#import <stddef.h>
#import <stdint.h>

typedef void (*DSQDiscoveryCallback)(int32_t, int32_t, int32_t, void *);
typedef void (*DSQFrameCallback)(int32_t, uint32_t, uint32_t, uint32_t, uint8_t *, size_t, void *);
typedef void (*DSQRecordingCallback)(int32_t, int32_t, uint64_t, uint64_t, void *);
typedef struct {
    uint64_t frameIndex;
    uint64_t displayTimeTicks;
    int32_t frameStatus;
    float scaleFactor;
    float contentScale;
    int32_t contentX;
    int32_t contentY;
    uint32_t contentWidth;
    uint32_t contentHeight;
    int32_t boundingX;
    int32_t boundingY;
    uint32_t boundingWidth;
    uint32_t boundingHeight;
} DSQFrameMetadata;
typedef int32_t (*DSQFrameMetadataCallback)(uint64_t, const DSQFrameMetadata *, void *);
typedef struct {
    uint64_t frameIndex;
    int32_t kind;
    float x;
    float y;
    float width;
    float height;
    float confidence;
} DSQVisionObservation;
typedef int32_t (*DSQVisionObservationCallback)(uint64_t, const DSQVisionObservation *, void *);
typedef struct {
    uint64_t frameIndex;
    float x;
    float y;
    float width;
    float height;
} DSQTemporalObservation;
typedef int32_t (*DSQTemporalObservationCallback)(uint64_t, const DSQTemporalObservation *, void *);
typedef struct {
    uint64_t frameIndex;
    int32_t kind;
    int32_t x;
    int32_t y;
    uint32_t width;
    uint32_t height;
} DSQAccessibilityObservation;
typedef int32_t (*DSQAccessibilityObservationCallback)(uint64_t, const DSQAccessibilityObservation *, void *);

int32_t dropsquash_secure_share_bridge_abi_version(void);
void dropsquash_secure_share_bridge_discover(void *, DSQDiscoveryCallback);
void dropsquash_secure_share_bridge_release_frame(uint8_t *);
void dropsquash_secure_share_bridge_capture_window_frame(uint32_t, int32_t, void *, DSQFrameCallback);
uint64_t dropsquash_secure_share_bridge_start_strict_recording(
    uint32_t, uint32_t, uint32_t, const char *, void *, DSQRecordingCallback
);
void dropsquash_secure_share_bridge_stop_recording(uint64_t);
uint64_t dropsquash_secure_share_bridge_start_strict_recording_with_metadata(
    uint32_t, uint32_t, uint32_t, const char *, void *, DSQRecordingCallback, DSQFrameMetadataCallback
);
uint64_t dropsquash_secure_share_bridge_start_attested_strict_recording(
    uint32_t, int32_t, int32_t, int32_t, uint32_t, uint32_t, uint32_t, uint32_t, const char *, void *,
    DSQRecordingCallback, DSQFrameMetadataCallback
);
uint64_t dropsquash_secure_share_bridge_start_attested_strict_recording_with_observations(
    uint32_t, int32_t, int32_t, int32_t, uint32_t, uint32_t, uint32_t, uint32_t, const char *, void *,
    DSQRecordingCallback, DSQFrameMetadataCallback, DSQVisionObservationCallback
);
uint64_t dropsquash_secure_share_bridge_start_attested_strict_recording_with_all_observations(
    uint32_t, int32_t, int32_t, int32_t, uint32_t, uint32_t, uint32_t, uint32_t, const char *, void *,
    DSQRecordingCallback, DSQFrameMetadataCallback, DSQVisionObservationCallback, DSQAccessibilityObservationCallback
);
uint64_t dropsquash_secure_share_bridge_start_attested_strict_recording_with_temporal_observations(
    uint32_t, int32_t, int32_t, int32_t, uint32_t, uint32_t, uint32_t, uint32_t, const char *, void *,
    DSQRecordingCallback, DSQFrameMetadataCallback, DSQVisionObservationCallback,
    DSQAccessibilityObservationCallback, DSQTemporalObservationCallback
);
