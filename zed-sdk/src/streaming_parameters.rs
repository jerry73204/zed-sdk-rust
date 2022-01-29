use crate::StreamingCodec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StreamingParameters {
    pub codec: StreamingCodec,
    pub bitrate: usize,
    pub port: u16,
    pub gop_size: Option<u8>,
    pub adaptative_bitrate: bool,
    pub chunk_size: usize,
    pub target_framerate: Option<usize>,
}

impl Default for StreamingParameters {
    fn default() -> Self {
        Self {
            codec: StreamingCodec::SL_STREAMING_CODEC_H265,
            bitrate: 80000,
            port: 30000,
            gop_size: None,
            adaptative_bitrate: false,
            chunk_size: 16084,
            target_framerate: None,
        }
    }
}
