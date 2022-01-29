use crate::common::*;

pub(crate) fn osstr_to_cstr<S>(from: S) -> CString
where
    S: AsRef<OsStr>,
{
    let from = from.as_ref();

    cfg_if! {
        if #[cfg(unix)] {
            use os::unix::ffi::OsStrExt;
            CString::new(from.as_bytes()).unwrap()
        } else if #[cfg(wasm)] {
            use os::wasi::ffi::OsStrExt;
            CString::new(from.as_bytes()).unwrap()
        } else if #[cfg(windows)] {
            use os::windows::ffi::OsStrExt;
            todo!();
        } else {
            panic!("");
        }
    }
}

macro_rules! declare_marker {
    ($vis:vis $name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name {
            _private: [u8; 0]
        }
    };
}

pub(crate) use declare_marker;
