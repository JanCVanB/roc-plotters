use roc_std::RocStr;

extern "C" {
    #[link_name = "roc__configForHost_1_exposed"]
    pub fn roc_config() -> Config;
}

/// The exact order of this struct's fields matters,
/// in order to successfully receive a Roc Record.
#[repr(C)]
pub struct Config {
    pub outputFilePath: RocStr,
    pub subtitle: RocStr,
    pub title: RocStr,
    pub height: u32,
    pub width: u32,
}
