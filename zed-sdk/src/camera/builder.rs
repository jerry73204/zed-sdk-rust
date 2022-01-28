use super::recording_state as rs;
use crate::{
    common::*, error::code_to_result, utils::osstr_to_cstr, Camera, CoordinateSystem, DepthMode,
    FlipMode, InitParameters, InputType, Resolution, Result, Unit,
};

#[derive(Debug, Clone)]
pub struct CameraBuilder<'a> {
    pub output_file: Option<Cow<'a, Path>>,
    pub opt_settings_path: Option<Cow<'a, Path>>,
    pub opencv_calib_path: Option<Cow<'a, Path>>,
    pub params: InitParameters,
}

impl<'a> CameraBuilder<'a> {
    pub fn new() -> Self {
        Self {
            output_file: None,
            opt_settings_path: None,
            opencv_calib_path: None,
            params: Default::default(),
        }
    }

    pub fn fps<N>(self, fps: N) -> Self
    where
        N: Into<Option<u32>>,
    {
        let fps = fps.into().map(|fps| fps as c_int).unwrap_or(0);
        Self {
            params: InitParameters {
                camera_fps: fps,
                ..self.params
            },
            ..self
        }
    }

    pub fn resolution(self, resolution: Resolution) -> Self {
        Self {
            params: InitParameters {
                resolution,
                ..self.params
            },
            ..self
        }
    }

    pub fn flip_mode(self, flip_mode: FlipMode) -> Self {
        Self {
            params: InitParameters {
                camera_image_flip: flip_mode,
                ..self.params
            },
            ..self
        }
    }

    pub fn depth_mode(self, depth_mode: DepthMode) -> Self {
        Self {
            params: InitParameters {
                depth_mode,
                ..self.params
            },
            ..self
        }
    }

    pub fn unit(self, unit: Unit) -> Self {
        Self {
            params: InitParameters {
                coordinate_unit: unit,
                ..self.params
            },
            ..self
        }
    }

    pub fn coordinate_system(self, sys: CoordinateSystem) -> Self {
        Self {
            params: InitParameters {
                coordinate_system: sys,
                ..self.params
            },
            ..self
        }
    }

    pub fn camera_self_calb(self, yes: bool) -> Self {
        Self {
            params: InitParameters {
                camera_disable_self_calib: !yes,
                ..self.params
            },
            ..self
        }
    }

    pub fn output_file<P>(self, path: P) -> Self
    where
        P: Into<Cow<'a, Path>>,
    {
        Self {
            output_file: Some(path.into()),
            ..self
        }
    }

    pub fn opt_settings_path<P>(self, path: P) -> Self
    where
        P: Into<Cow<'a, Path>>,
    {
        Self {
            opt_settings_path: Some(path.into()),
            ..self
        }
    }

    pub fn opencv_calib_path<P>(self, path: P) -> Self
    where
        P: Into<Cow<'a, Path>>,
    {
        Self {
            opencv_calib_path: Some(path.into()),
            ..self
        }
    }

    pub fn open_svo<P>(self, svo_path: P) -> Result<Camera<rs::Inactive>>
    where
        P: AsRef<Path>,
    {
        let Self {
            output_file,
            opt_settings_path,
            opencv_calib_path,
            params,
        } = self;

        let mut params = InitParameters {
            input_type: InputType::SL_INPUT_TYPE_SVO,
            ..params
        };
        let output_file: Option<_> = output_file.map(|path| osstr_to_cstr(path.as_ref()));
        let opt_settings_path: Option<_> =
            opt_settings_path.map(|path| osstr_to_cstr(path.as_ref()));
        let opencv_calib_path: Option<_> =
            opencv_calib_path.map(|path| osstr_to_cstr(path.as_ref()));

        let svo_path = osstr_to_cstr(svo_path.as_ref());

        let code = unsafe {
            sys::sl_open_camera(
                0,
                &mut params as *mut _,
                svo_path.as_ptr(),
                ptr::null(),
                0,
                output_file.map(|path| path.as_ptr()).unwrap_or(ptr::null()),
                opt_settings_path
                    .map(|path| path.as_ptr())
                    .unwrap_or(ptr::null()),
                opencv_calib_path
                    .map(|path| path.as_ptr())
                    .unwrap_or(ptr::null()),
            )
        };
        code_to_result(code as u32)?;

        todo!("allocate new camera id");
    }

    pub fn open_stream<A>(self, addr: A) -> Result<Camera<rs::Inactive>>
    where
        A: Into<SocketAddr>,
    {
        let Self {
            output_file,
            opt_settings_path,
            opencv_calib_path,
            params,
        } = self;

        let mut params = InitParameters {
            input_type: InputType::SL_INPUT_TYPE_STREAM,
            ..params
        };
        let output_file: Option<_> = output_file.map(|path| osstr_to_cstr(path.as_ref()));
        let opt_settings_path: Option<_> =
            opt_settings_path.map(|path| osstr_to_cstr(path.as_ref()));
        let opencv_calib_path: Option<_> =
            opencv_calib_path.map(|path| osstr_to_cstr(path.as_ref()));

        let addr = addr.into();
        let ip = CString::new(addr.ip().to_string()).unwrap();
        let port = addr.port();

        let code = unsafe {
            sys::sl_open_camera(
                0,
                &mut params as *mut _,
                ptr::null(),
                ip.as_ptr(),
                port as c_int,
                output_file.map(|path| path.as_ptr()).unwrap_or(ptr::null()),
                opt_settings_path
                    .map(|path| path.as_ptr())
                    .unwrap_or(ptr::null()),
                opencv_calib_path
                    .map(|path| path.as_ptr())
                    .unwrap_or(ptr::null()),
            )
        };
        code_to_result(code as u32)?;

        todo!("allocate new camera id");
    }

    pub fn open_usb(self, id: u8) -> Result<Camera<rs::Inactive>> {
        let Self {
            output_file,
            opt_settings_path,
            opencv_calib_path,
            params,
        } = self;

        let mut params = InitParameters {
            camera_device_id: id as i32,
            input_type: InputType::SL_INPUT_TYPE_USB,
            ..params
        };
        let output_file: Option<_> = output_file.map(|path| osstr_to_cstr(path.as_ref()));
        let opt_settings_path: Option<_> =
            opt_settings_path.map(|path| osstr_to_cstr(path.as_ref()));
        let opencv_calib_path: Option<_> =
            opencv_calib_path.map(|path| osstr_to_cstr(path.as_ref()));

        let code = unsafe {
            sys::sl_open_camera(
                id as i32,
                &mut params as *mut _,
                ptr::null(),
                ptr::null(),
                0,
                output_file.map(|path| path.as_ptr()).unwrap_or(ptr::null()),
                opt_settings_path
                    .map(|path| path.as_ptr())
                    .unwrap_or(ptr::null()),
                opencv_calib_path
                    .map(|path| path.as_ptr())
                    .unwrap_or(ptr::null()),
            )
        };
        code_to_result(code as u32)?;

        Ok(Camera::new(id))
    }
}

impl<'a> Default for CameraBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
