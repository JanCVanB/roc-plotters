use roc_std::{RocList, RocStr};

#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct P2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct Line {
    pub name: RocStr,
    pub points: RocList<P2>,
    pub color: Color,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct Config {
    pub lines: RocList<Line>,
    pub output_file_path: RocStr,
    pub subtitle: RocStr,
    pub title: RocStr,
    pub height: u32,
    pub width: u32,
}

pub fn roc_config() -> Config {
    extern "C" {
        #[link_name = "roc__configForHost_1_exposed_generic"]
        fn call(_: &mut Config);
    }
    let mut config = Config::default();
    unsafe { call(&mut config) };
    dbg!(config)
}
