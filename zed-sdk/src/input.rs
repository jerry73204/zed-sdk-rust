use crate::{common::*, InputType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Input<'a> {
    Svo(Cow<'a, Path>),
    Stream(SocketAddr),
    Usb(c_int),
}

impl<'a> Input<'a> {
    pub fn svo_file<P>(path: P) -> Self
    where
        P: Into<Cow<'a, Path>>,
    {
        Self::Svo(path.into())
    }

    pub fn stream<A>(addr: A) -> Self
    where
        A: Into<SocketAddr>,
    {
        Self::Stream(addr.into())
    }

    pub fn usb<T>(id: T) -> Self
    where
        T: ToPrimitive,
    {
        Self::Usb(<c_int as NumCast>::from(id).unwrap())
    }

    pub fn input_type(&self) -> InputType {
        match self {
            Input::Svo(_) => InputType::SL_INPUT_TYPE_SVO,
            Input::Stream(_) => InputType::SL_INPUT_TYPE_STREAM,
            Input::Usb(_) => InputType::SL_INPUT_TYPE_USB,
        }
    }
}

impl<'a> From<&'a Path> for Input<'a> {
    fn from(path: &'a Path) -> Self {
        Self::svo_file(path)
    }
}

impl<'a> From<PathBuf> for Input<'a> {
    fn from(path: PathBuf) -> Self {
        Self::svo_file(path)
    }
}

impl<'a> From<SocketAddr> for Input<'a> {
    fn from(addr: SocketAddr) -> Self {
        Self::stream(addr)
    }
}

impl<'a> From<SocketAddrV4> for Input<'a> {
    fn from(addr: SocketAddrV4) -> Self {
        Self::stream(addr)
    }
}

impl<'a> From<SocketAddrV6> for Input<'a> {
    fn from(addr: SocketAddrV6) -> Self {
        Self::stream(addr)
    }
}

impl<'a> From<c_int> for Input<'a> {
    fn from(id: c_int) -> Self {
        Self::usb(id)
    }
}
