#![allow(non_snake_case)]

use roc_std::RocStr;

extern "C" {
    #[link_name = "roc__optionsForHost_1_exposed"]
    pub fn roc_options() -> Options;
}

/// The exact order of this struct's fields matters,
/// in order to successfully receive a Roc Record.
#[repr(C)]
pub struct Options {
    pub outputFilePath: RocStr,
    pub subtitle: RocStr,
    pub title: RocStr,
    pub height: u32,
    pub width: u32,
}
