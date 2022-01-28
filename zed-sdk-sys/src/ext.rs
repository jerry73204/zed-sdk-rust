use crate::{
    SL_InitParameters, SL_RuntimeParameters, SL_CAMERA_STATE, SL_COORDINATE_SYSTEM, SL_DEPTH_MODE,
    SL_ERROR_CODE, SL_FLIP_MODE, SL_INPUT_TYPE, SL_MODEL,
    SL_REFERENCE_FRAME_SL_REFERENCE_FRAME_CAMERA, SL_RESOLUTION, SL_UNIT,
};
use std::fmt::{self, Display};

impl Default for SL_InitParameters {
    fn default() -> Self {
        Self {
            input_type: SL_INPUT_TYPE::SL_INPUT_TYPE_USB,
            resolution: SL_RESOLUTION::SL_RESOLUTION_HD720,
            camera_fps: 0,
            camera_device_id: 0,
            camera_image_flip: SL_FLIP_MODE::SL_FLIP_MODE_AUTO,
            camera_disable_self_calib: false,
            enable_right_side_measure: false,
            svo_real_time_mode: false,
            depth_mode: SL_DEPTH_MODE::SL_DEPTH_MODE_ULTRA,
            depth_stabilization: true,
            depth_minimum_distance: -1.0,
            depth_maximum_distance: -1.0,
            coordinate_unit: SL_UNIT::SL_UNIT_MILLIMETER,
            coordinate_system: SL_COORDINATE_SYSTEM::SL_COORDINATE_SYSTEM_IMAGE,
            sdk_gpu_id: -1,
            sdk_verbose: 0,
            sensors_required: false,
            enable_image_enhancement: true,
            open_timeout_sec: 5.0,
        }
    }
}

impl Default for SL_RuntimeParameters {
    fn default() -> Self {
        Self {
            sensing_mode: crate::SL_SENSING_MODE::SL_SENSING_MODE_STANDARD,
            reference_frame: SL_REFERENCE_FRAME_SL_REFERENCE_FRAME_CAMERA,
            enable_depth: true,
            confidence_threshold: 100,
            texture_confidence_threshold: 100,
        }
    }
}

impl Display for SL_ERROR_CODE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            SL_ERROR_CODE::SL_ERROR_CODE_SUCCESS => "SUCCESS",
            SL_ERROR_CODE::SL_ERROR_CODE_FAILURE => "FAILURE",
            SL_ERROR_CODE::SL_ERROR_CODE_NO_GPU_COMPATIBLE => "NO_GPU_COMPATIBLE",
            SL_ERROR_CODE::SL_ERROR_CODE_NOT_ENOUGH_GPU_MEMORY => "NOT_ENOUGH_GPU_MEMORY",
            SL_ERROR_CODE::SL_ERROR_CODE_CAMERA_NOT_DETECTED => "CAMERA_NOT_DETECTED",
            SL_ERROR_CODE::SL_ERROR_CODE_SENSORS_NOT_INITIALIZED => "SENSORS_NOT_INITIALIZED",
            SL_ERROR_CODE::SL_ERROR_CODE_SENSORS_NOT_AVAILABLE => "SENSORS_NOT_AVAILABLE",
            SL_ERROR_CODE::SL_ERROR_CODE_INVALID_RESOLUTION => "INVALID_RESOLUTION",
            SL_ERROR_CODE::SL_ERROR_CODE_LOW_USB_BANDWIDTH => "LOW_USB_BANDWIDTH",
            SL_ERROR_CODE::SL_ERROR_CODE_CALIBRATION_FILE_NOT_AVAILABLE => {
                "CALIBRATION_FILE_NOT_AVAILABLE"
            }
            SL_ERROR_CODE::SL_ERROR_CODE_INVALID_CALIBRATION_FILE => "INVALID_CALIBRATION_FILE",
            SL_ERROR_CODE::SL_ERROR_CODE_INVALID_SVO_FILE => "INVALID_SVO_FILE",
            SL_ERROR_CODE::SL_ERROR_CODE_SVO_RECORDING_ERROR => "SVO_RECORDING_ERROR",
            SL_ERROR_CODE::SL_ERROR_CODE_SVO_UNSUPPORTED_COMPRESSION => {
                "SVO_UNSUPPORTED_COMPRESSION"
            }
            SL_ERROR_CODE::SL_ERROR_CODE_END_OF_SVOFILE_REACHED => "END_OF_SVOFILE_REACHED",
            SL_ERROR_CODE::SL_ERROR_CODE_INVALID_COORDINATE_SYSTEM => "INVALID_COORDINATE_SYSTEM",
            SL_ERROR_CODE::SL_ERROR_CODE_INVALID_FIRMWARE => "INVALID_FIRMWARE",
            SL_ERROR_CODE::SL_ERROR_CODE_INVALID_FUNCTION_PARAMETERS => {
                "INVALID_FUNCTION_PARAMETERS"
            }
            SL_ERROR_CODE::SL_ERROR_CODE_CUDA_ERROR => "CUDA_ERROR",
            SL_ERROR_CODE::SL_ERROR_CODE_CAMERA_NOT_INITIALIZED => "CAMERA_NOT_INITIALIZED",
            SL_ERROR_CODE::SL_ERROR_CODE_NVIDIA_DRIVER_OUT_OF_DATE => "NVIDIA_DRIVER_OUT_OF_DATE",
            SL_ERROR_CODE::SL_ERROR_CODE_INVALID_FUNCTION_CALL => "INVALID_FUNCTION_CALL",
            SL_ERROR_CODE::SL_ERROR_CODE_CORRUPTED_SDK_INSTALLATION => "CORRUPTED_SDK_INSTALLATION",
            SL_ERROR_CODE::SL_ERROR_CODE_INCOMPATIBLE_SDK_VERSION => "INCOMPATIBLE_SDK_VERSION",
            SL_ERROR_CODE::SL_ERROR_CODE_INVALID_AREA_FILE => "INVALID_AREA_FILE",
            SL_ERROR_CODE::SL_ERROR_CODE_INCOMPATIBLE_AREA_FILE => "INCOMPATIBLE_AREA_FILE",
            SL_ERROR_CODE::SL_ERROR_CODE_CAMERA_FAILED_TO_SETUP => "CAMERA_FAILED_TO_SETUP",
            SL_ERROR_CODE::SL_ERROR_CODE_CAMERA_DETECTION_ISSUE => "CAMERA_DETECTION_ISSUE",
            SL_ERROR_CODE::SL_ERROR_CODE_CANNOT_START_CAMERA_STREAM => "CANNOT_START_CAMERA_STREAM",
            SL_ERROR_CODE::SL_ERROR_CODE_NO_GPU_DETECTED => "NO_GPU_DETECTED",
            SL_ERROR_CODE::SL_ERROR_CODE_PLANE_NOT_FOUND => "PLANE_NOT_FOUND",
            SL_ERROR_CODE::SL_ERROR_CODE_MODULE_NOT_COMPATIBLE_WITH_CAMERA => {
                "MODULE_NOT_COMPATIBLE_WITH_CAMERA"
            }
            SL_ERROR_CODE::SL_ERROR_CODE_MOTION_SENSORS_REQUIRED => "MOTION_SENSORS_REQUIRED",
            SL_ERROR_CODE::SL_ERROR_CODE_MODULE_NOT_COMPATIBLE_WITH_CUDA_VERSION => {
                "MODULE_NOT_COMPATIBLE_WITH_CUDA_VERSION"
            }
        };
        write!(f, "{}", text)
    }
}

impl Display for SL_MODEL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            SL_MODEL::SL_MODEL_ZED => "ZED",
            SL_MODEL::SL_MODEL_ZED_M => "ZED M",
            SL_MODEL::SL_MODEL_ZED2 => "ZED 2",
            SL_MODEL::SL_MODEL_ZED2i => "ZED 2i",
        };
        write!(f, "{}", name)
    }
}

impl Display for SL_CAMERA_STATE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self {
            SL_CAMERA_STATE::SL_CAMERA_STATE_AVAILABLE => "available",
            SL_CAMERA_STATE::SL_CAMERA_STATE_NOT_AVAILABLE => "not available",
        };
        write!(f, "{}", state)
    }
}
