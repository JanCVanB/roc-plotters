use roc_std::{RocList, RocStr};

#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct P2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct Bounds {
    pub xMax: f64,
    pub xMin: f64,
    pub yMax: f64,
    pub yMin: f64,
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
pub struct Fonts {
    pub subtitleFamily: RocStr,
    pub titleFamily: RocStr,
    pub subtitleSize: u32,
    pub titleSize: u32,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct Labels {
    pub xCount: usize,
    pub yCount: usize,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct Layout {
    pub chartMargin: u32,
    pub labelArea: u32,
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
    pub bounds: Bounds,
    pub fonts: Fonts,
    pub labels: Labels,
    pub lines: RocList<Line>,
    pub output_file_path: RocStr,
    pub subtitle: RocStr,
    pub title: RocStr,
    pub height: u32,
    pub layout: Layout,
    pub width: u32,
}

pub fn roc_config() -> Config {
    extern "C" {
        #[link_name = "roc__configForHost_1_exposed_generic"]
        fn call(_: &mut Config);
    }
    let mut config = Config::default();
    unsafe { call(&mut config) }
    config
}
