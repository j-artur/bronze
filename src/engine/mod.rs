pub mod context;
pub mod engine;
pub mod game;
pub mod graphics;
pub mod resources;
pub mod timer;
pub mod window;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        crate::engine::Color($r, $g, $b, 255)
    };
}

#[allow(unused)]
macro_rules! rgba {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        crate::engine::Color($r, $g, $b, $a)
    };
}

#[allow(unused)]
pub(crate) use {rgb, rgba};

macro_rules! u16str {
    ($s:expr) => {{
        use std::ffi::OsStr;
        use std::os::windows::prelude::OsStrExt;

        OsStr::new($s)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>()
            .as_ptr()
    }};
}

pub(crate) use u16str;
