use crate::{common::*, ErrorCode};

#[derive(Debug, Clone, PartialEq, Eq, Hash, thiserror::Error)]
pub enum Error {
    #[error("ZED error: {0}")]
    Code(ErrorCode),
    #[error("{0}")]
    Desc(Cow<'static, str>),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) fn code_to_result(code: u32) -> Result<()> {
    let code: ErrorCode = unsafe { mem::transmute(code) };

    match code {
        sys::SL_ERROR_CODE::SL_ERROR_CODE_SUCCESS => Ok(()),
        code => Err(Error::Code(code)),
    }
}

macro_rules! ensure {
    ($cond:expr, $($format:tt)*) => {
        if !$cond {
            return Err(crate::error::Error::Desc(format!( $($format)* ).into()));
        }
    };
}
pub(crate) use ensure;
