use crate::config::roc_config;
use crate::plot::plot;

pub mod config;
pub mod memory;
pub mod plot;

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    let config = roc_config();
    plot(config).unwrap();
    0
}