use roc_std::{ RocList, RocStr };

#[derive(Default, Debug)]
#[repr(C)]
pub struct Config {
    pub outputFilePath: RocStr,
    pub subtitle: RocStr,
    pub title: RocStr,
    pub x: RocList<i32>,
    pub y1: RocList<i32>,
    pub y2: RocList<i32>,
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
