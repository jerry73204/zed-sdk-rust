use super::{input_source as is, recording_state as rs, streaming_state as ss};
use crate::{
    common::*, ensure, error::code_to_result, utils::osstr_to_cstr, CalibrationParameters, Mem,
    Model, RecordingParams, Result, RuntimeParameters, StreamingParameters, VideoSettings, View,
};
use std::os::raw::{c_uint, c_ulonglong, c_ushort};

#[derive(Debug)]
pub struct Camera<I, R, S> {
    inner: Inner,
    _phantom: PhantomData<(I, R, S)>,
}

impl<I, R, S> Camera<I, R, S> {
    pub fn id(&mut self) -> c_int {
        self.inner.id
    }

    pub fn resolution(&mut self) -> (usize, usize) {
        (self.width(), self.height())
    }

    pub fn height(&mut self) -> usize {
        unsafe { sys::sl_get_height(self.id()) as usize }
    }

    pub fn width(&mut self) -> usize {
        unsafe { sys::sl_get_width(self.id()) as usize }
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

impl<I> Camera<I, rs::Inactive, ss::Inactive> {
    pub(crate) fn new(id: c_int) -> Self {
        Self {
            inner: Inner {
                id,
                recording_state: RecordingState::Inactive,
                streaming_state: StreamingState::Inactive,
            },
            _phantom: PhantomData,
        }
    }

    pub fn close(mut self) {
        unsafe {
            let id = self.id();
            sys::sl_close_camera(id);
            self.inner.recording_state = RecordingState::Closed;
            self.inner.streaming_state = StreamingState::Closed;
        }
    }
}

impl<I, S> Camera<I, rs::Inactive, S> {
    pub fn enable_recording<P>(
        mut self,
        output_file: P,
        params: RecordingParams,
    ) -> Result<Camera<I, rs::Recording, S>>
    where
        P: AsRef<Path>,
    {
        let output_file = osstr_to_cstr(output_file.as_ref());
        let id = self.id();

        let code = unsafe {
            sys::sl_enable_recording(
                id,
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
}

impl<I, S> Camera<I, rs::Recording, S> {
    pub fn disable_recording(mut self) -> Camera<I, rs::Inactive, S> {
        let id = self.id();
        unsafe {
            sys::sl_disable_recording(id);
        }

        Camera {
            inner: Inner {
                recording_state: RecordingState::Inactive,
                ..self.inner
            },
            _phantom: PhantomData,
        }
    }

    pub fn pause_recording(mut self) -> Camera<I, rs::Paused, S> {
        let id = self.id();
        unsafe {
            sys::sl_pause_recording(id, true);
        }

        Camera {
            inner: Inner {
                recording_state: RecordingState::Paused,
                ..self.inner
            },
            _phantom: PhantomData,
        }
    }

    pub fn grab(&mut self, mut runtime: RuntimeParameters) -> Result<GrabHandle<'_, I, S>> {
        let id = self.id();
        let code = unsafe { sys::sl_grab(id, &mut runtime as *mut _) };
        code_to_result(code as u32)?;
        Ok(GrabHandle { camera: self })
    }
}

impl<I, S> Camera<I, rs::Paused, S> {
    pub fn resume_recording(mut self) -> Camera<I, rs::Recording, S> {
        let id = self.id();
        unsafe {
            sys::sl_pause_recording(id, false);
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

impl<I, R> Camera<I, R, ss::Inactive> {
    pub fn enable_streaming(
        mut self,
        params: StreamingParameters,
    ) -> Result<Camera<I, R, ss::Streaming>> {
        let id = self.id();
        let StreamingParameters {
            codec,
            bitrate,
            port,
            gop_size,
            adaptative_bitrate,
            chunk_size,
            target_framerate,
        } = params;

        let code = unsafe {
            sys::sl_enable_streaming(
                id,
                codec,
                bitrate as c_uint,
                port as c_ushort,
                gop_size.map(|size| size as c_int).unwrap_or(-1),
                adaptative_bitrate as c_int,
                chunk_size as c_int,
                target_framerate.map(|fps| fps as c_int).unwrap_or(0),
            )
        };
        code_to_result(code as u32)?;

        Ok(Camera {
            inner: Inner {
                streaming_state: StreamingState::Streaming,
                ..self.inner
            },
            _phantom: PhantomData,
        })
    }
}

impl<I, R> Camera<I, R, ss::Streaming> {
    pub fn disable_streaming(mut self) -> Camera<I, R, ss::Inactive> {
        let id = self.id();
        unsafe {
            sys::sl_disable_streaming(id);
        }

        Camera {
            inner: Inner {
                streaming_state: StreamingState::Inactive,
                ..self.inner
            },
            _phantom: PhantomData,
        }
    }
}

impl<R, S> Camera<is::SVO, R, S> {
    pub fn svo_position(&mut self) -> usize {
        let id = self.id();
        unsafe { sys::sl_get_svo_position(id) as usize }
    }

    pub fn set_svo_position(&mut self, pos: usize) {
        let id = self.id();
        unsafe {
            sys::sl_set_svo_position(id, pos as c_int);
        }
    }

    pub fn num_frames(&mut self) -> usize {
        let id = self.id();
        unsafe { sys::sl_get_svo_number_of_frames(id) as usize }
    }
}

pub use grab::*;
mod grab {
    use super::*;

    pub struct GrabHandle<'a, I, S> {
        pub(super) camera: &'a mut Camera<I, rs::Recording, S>,
    }

    impl<'a, I, S> GrabHandle<'a, I, S> {
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
                    id,
                    buffer.as_mut_ptr(),
                    type_,
                    mem,
                    width as c_int,
                    height as c_int,
                )
            };
            code_to_result(code as u32)
        }

        pub fn retrieve_image_to_vec<R>(
            &mut self,
            type_: View,
            mem: Mem,
            wh: R,
        ) -> Result<Vec<c_int>>
        where
            R: Into<Option<(usize, usize)>>,
        {
            let wh = wh.into().unwrap_or_else(|| self.camera.resolution());
            let (width, height) = wh;
            let mut buffer: Vec<c_int> = vec![0; width * height];
            self.retrieve_image(type_, mem, (width, height), &mut buffer)?;
            Ok(buffer)
        }

        pub fn image_timestamp(&mut self) -> c_ulonglong {
            let id = self.camera.id();
            unsafe { sys::sl_get_image_timestamp(id) }
        }

        pub fn current_timestamp(&mut self) -> c_ulonglong {
            let id = self.camera.id();
            unsafe { sys::sl_get_current_timestamp(id) }
        }

        pub fn save_current_image<P>(&mut self, view: View, output_file: P) -> Result<()>
        where
            P: AsRef<Path>,
        {
            let id = self.camera.id();
            let output_file = osstr_to_cstr(output_file.as_ref());

            let code = unsafe { sys::sl_save_current_image(id, view, output_file.as_ptr()) };
            code_to_result(code as u32)
        }
    }
}

use inner::*;
mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct Inner {
        pub id: c_int,
        pub recording_state: RecordingState,
        pub streaming_state: StreamingState,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum RecordingState {
        Inactive,
        Recording,
        Paused,
        Closed,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum StreamingState {
        Inactive,
        Streaming,
        Closed,
    }

    impl Drop for Inner {
        fn drop(&mut self) {
            let id = self.id;

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

            match self.streaming_state {
                StreamingState::Inactive => {}
                StreamingState::Streaming => unsafe {
                    sys::sl_disable_streaming(id);
                    sys::sl_close_camera(id);
                },
                StreamingState::Closed => {}
            }
        }
    }
}
