use roc_std::{RocList, RocStr};

#[derive(Default, Debug)]
#[repr(C)]
pub struct P2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct Config {
    pub lines: RocList<RocList<P2>>,
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
    config
}
