use super::recording_state as rs;
use crate::{
    common::*, ensure, error::code_to_result, utils::osstr_to_cstr, CalibrationParameters, Mem,
    Model, RecordingParams, Result, RuntimeParameters, VideoSettings, View,
};
use std::os::raw::{c_uint, c_ulonglong};

#[derive(Debug)]
pub struct Camera<T> {
    inner: Inner,
    _phantom: PhantomData<T>,
}

impl Camera<rs::Inactive> {
    pub fn enable_recording<P>(
        mut self,
        output_file: P,
        params: RecordingParams,
    ) -> Result<Camera<rs::Recording>>
    where
        P: AsRef<Path>,
    {
        let output_file = osstr_to_cstr(output_file.as_ref());
        let id = self.id();

        let code = unsafe {
            sys::sl_enable_recording(
                id as c_int,
                output_file.as_ptr(),
                params.compression_mode,
                params.bitrate as c_uint,
                params.target_framerate as c_int,
                params.transcode_streaming_input,
            )
        };
        code_to_result(code as u32)?;

        Ok(Camera {
            inner: Inner {
                recording_state: RecordingState::Recording,
                ..self.inner
            },
            _phantom: PhantomData,
        })
    }

    pub fn close(mut self) {
        unsafe {
            let id = self.id();
            sys::sl_close_camera(id);
            self.inner.recording_state = RecordingState::Closed;
        }
    }

    pub(crate) fn new(id: u8) -> Self {
        Self {
            inner: Inner {
                id,
                recording_state: RecordingState::Inactive,
            },
            _phantom: PhantomData,
        }
    }
}

impl Camera<rs::Recording> {
    pub fn disable_recording(mut self) -> Camera<rs::Inactive> {
        let id = self.id();
        unsafe {
            sys::sl_disable_recording(id as c_int);
        }

        Camera {
            inner: Inner {
                recording_state: RecordingState::Inactive,
                ..self.inner
            },
            _phantom: PhantomData,
        }
    }

    pub fn pause_recording(mut self) -> Camera<rs::Paused> {
        let id = self.id();
        unsafe {
            sys::sl_pause_recording(id as c_int, true);
        }

        Camera {
            inner: Inner {
                recording_state: RecordingState::Paused,
                ..self.inner
            },
            _phantom: PhantomData,
        }
    }

    pub fn grab(&mut self, mut runtime: RuntimeParameters) -> Result<GrabHandle<'_>> {
        let id = self.id();
        let code = unsafe { sys::sl_grab(id as c_int, &mut runtime as *mut _) };
        code_to_result(code as u32)?;
        Ok(GrabHandle { camera: self })
    }
}

impl Camera<rs::Paused> {
    pub fn resume_recording(mut self) -> Camera<rs::Recording> {
        let id = self.id();
        unsafe {
            sys::sl_pause_recording(id as c_int, false);
        }

        Camera {
            inner: Inner {
                recording_state: RecordingState::Recording,
                ..self.inner
            },
            _phantom: PhantomData,
        }
    }
}

impl<T> Camera<T> {
    pub fn id(&mut self) -> c_int {
        self.inner.id as c_int
    }

    pub fn height(&mut self) -> c_int {
        unsafe { sys::sl_get_height(self.id()) }
    }

    pub fn width(&mut self) -> c_int {
        unsafe { sys::sl_get_width(self.id()) }
    }

    pub fn fps(&mut self) -> f32 {
        unsafe { sys::sl_get_camera_fps(self.id()) }
    }

    pub fn model(&mut self) -> Model {
        unsafe {
            let model = sys::sl_get_camera_model(self.id());
            mem::transmute(model)
        }
    }

    pub fn firmware(&mut self) -> c_int {
        unsafe { sys::sl_get_camera_firmware(self.id()) }
    }

    pub fn settings(&mut self, option: VideoSettings) -> Result<c_int> {
        let id = self.id();

        unsafe {
            let ret = sys::sl_get_camera_settings(id, option);
            ensure!(ret >= 0, "unable to get video settings");
            Ok(ret)
        }
    }

    pub fn calibration_parameters(&mut self, raw: bool) -> &CalibrationParameters {
        let id = self.id();

        unsafe {
            sys::sl_get_calibration_parameters(id, raw)
                .as_ref()
                .unwrap()
        }
    }
}

pub use grab::*;
mod grab {
    use super::*;

    pub struct GrabHandle<'a> {
        pub(super) camera: &'a mut Camera<rs::Recording>,
    }

    impl<'a> GrabHandle<'a> {
        pub fn retrieve_image<B>(
            &mut self,
            type_: View,
            mem: Mem,
            wh: (usize, usize),
            mut buffer: B,
        ) -> Result<()>
        where
            B: AsMut<[c_int]>,
        {
            let id = self.camera.id();
            let (width, height) = wh;
            let buffer = buffer.as_mut();
            ensure!(
                buffer.len() == width * height,
                "expect buffer size to be {} for {}x{}, but get {}",
                width * height,
                width,
                height,
                buffer.len()
            );

            let code = unsafe {
                sys::sl_retrieve_image(
                    id as c_int,
                    buffer.as_mut_ptr(),
                    type_,
                    mem,
                    width as c_int,
                    height as c_int,
                )
            };
            code_to_result(code as u32)
        }

        pub fn retrieve_image_to_vec(
            &mut self,
            type_: View,
            mem: Mem,
            wh: (usize, usize),
        ) -> Result<Vec<c_int>> {
            let (width, height) = wh;
            let mut buffer: Vec<c_int> = vec![0; width * height];
            self.retrieve_image(type_, mem, wh, &mut buffer)?;
            Ok(buffer)
        }

        pub fn image_timestamp(&mut self) -> c_ulonglong {
            let id = self.camera.id();
            unsafe { sys::sl_get_image_timestamp(id as c_int) }
        }

        pub fn current_timestamp(&mut self) -> c_ulonglong {
            let id = self.camera.id();
            unsafe { sys::sl_get_current_timestamp(id as c_int) }
        }

        pub fn save_current_image<P>(&mut self, view: View, output_file: P) -> Result<()>
        where
            P: AsRef<Path>,
        {
            let id = self.camera.id();
            let output_file = osstr_to_cstr(output_file.as_ref());

            let code =
                unsafe { sys::sl_save_current_image(id as c_int, view, output_file.as_ptr()) };
            code_to_result(code as u32)
        }
    }
}

use inner::*;
mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct Inner {
        pub id: u8,
        pub recording_state: RecordingState,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum RecordingState {
        Inactive,
        Recording,
        Paused,
        Closed,
    }

    impl Drop for Inner {
        fn drop(&mut self) {
            let id = self.id as c_int;

            match self.recording_state {
                RecordingState::Inactive => {}
                RecordingState::Recording => unsafe {
                    sys::sl_disable_recording(id);
                    sys::sl_close_camera(id);
                },
                RecordingState::Paused => unsafe {
                    sys::sl_disable_recording(id);
                    sys::sl_close_camera(id);
                },
                RecordingState::Closed => {}
            }
        }
    }
}
