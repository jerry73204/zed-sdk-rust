use crate::common::*;

pub struct RecordingParams {
    pub compression_mode: sys::SL_SVO_COMPRESSION_MODE,
    pub bitrate: u32,
    pub target_framerate: u32,
    pub transcode_streaming_input: bool,
}

impl Default for RecordingParams {
    fn default() -> Self {
        Self {
            compression_mode: sys::SL_SVO_COMPRESSION_MODE::SL_SVO_COMPRESSION_MODE_H264,
            bitrate: 0,
            target_framerate: 0,
            transcode_streaming_input: false,
        }
    }
}
