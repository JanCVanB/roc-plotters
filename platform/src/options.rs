#![allow(non_snake_case)]

use roc_std::RocStr;

extern "C" {
    #[link_name = "roc__optionsForHost_1_exposed"]
    pub fn roc_options() -> Options;
}

#[repr(C)]
pub struct Options {
    pub outputFilePath: RocStr,
    pub subtitle: RocStr,
    pub title: RocStr,
}
