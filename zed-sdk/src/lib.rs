mod camera;
mod common;
pub mod consts;
pub mod error;
mod input;
mod recording;
mod streaming_parameters;
mod utils;

use crate::common::*;
pub use camera::*;
pub use consts::*;
pub use error::*;
pub use input::*;
pub use recording::*;
pub use streaming_parameters::*;

pub type ErrorCode = sys::SL_ERROR_CODE;
pub type DeviceProperties = sys::SL_DeviceProperties;
pub type InitParameters = sys::SL_InitParameters;
pub type Resolution = sys::SL_RESOLUTION;
pub type DepthMode = sys::SL_DEPTH_MODE;
pub type InputType = sys::SL_INPUT_TYPE;
pub type RuntimeParameters = sys::SL_RuntimeParameters;
pub type Model = sys::SL_MODEL;
pub type VideoSettings = sys::SL_VIDEO_SETTINGS;
pub type View = sys::SL_VIEW;
pub type Mem = sys::SL_MEM;
pub type CalibrationParameters = sys::SL_CalibrationParameters;
pub type CameraParameters = sys::SL_CameraParameters;
pub type FlipMode = sys::SL_FLIP_MODE;
pub type Vector4 = sys::SL_Vector4;
pub type Unit = sys::SL_UNIT;
pub type CoordinateSystem = sys::SL_COORDINATE_SYSTEM;
pub type StreamingCodec = sys::SL_STREAMING_CODEC;

pub fn get_device_list() -> Vec<DeviceProperties> {
    unsafe {
        let mut nb_devices: c_int = 0;
        let mut device_list: Vec<sys::SL_DeviceProperties> = Vec::with_capacity(MAX_CAMERA_PLUGIN);
        sys::sl_get_device_list(device_list.as_mut_ptr(), &mut nb_devices as *mut c_int);
        device_list.set_len(nb_devices as usize);
        mem::transmute(device_list)
    }
}
