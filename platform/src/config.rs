use roc_std::{ RocList, RocStr };

#[derive(Default, Debug)]
#[repr(C)]
pub struct Config {
    pub outputFilePath: RocStr,
    pub points1: RocList<RocList<i32>>,
    pub points2: RocList<RocList<i32>>,
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
