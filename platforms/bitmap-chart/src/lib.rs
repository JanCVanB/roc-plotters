use crate::options::roc_options;
use crate::plot::plot;

pub mod memory;
pub mod options;
pub mod plot;

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    let options;
    unsafe {
        options = roc_options();
    }
    plot(options).unwrap();
    0
}