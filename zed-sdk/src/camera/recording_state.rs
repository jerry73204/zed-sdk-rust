macro_rules! declare_marker {
    ($vis:vis $name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name {
            _private: [u8; 0]
        }
    };
}

declare_marker!(pub Inactive);
declare_marker!(pub Recording);
declare_marker!(pub Paused);
