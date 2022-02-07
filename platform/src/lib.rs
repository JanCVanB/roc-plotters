#![allow(non_snake_case)]

use core::ffi::c_void;
use plotters::prelude::*;
use roc_std::RocStr;
use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    #[link_name = "roc__optionsForHost_1_exposed"]
    fn roc_options() -> Options;
}

#[repr(C)]
struct Options {
    outputFilePath: RocStr,
    title: RocStr,
}

#[no_mangle]
pub unsafe extern "C" fn roc_alloc(size: usize, _alignment: u32) -> *mut c_void {
    return libc::malloc(size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_realloc(
    c_ptr: *mut c_void,
    new_size: usize,
    _old_size: usize,
    _alignment: u32,
) -> *mut c_void {
    return libc::realloc(c_ptr, new_size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_dealloc(c_ptr: *mut c_void, _alignment: u32) {
    return libc::free(c_ptr);
}

#[no_mangle]
pub unsafe extern "C" fn roc_panic(c_ptr: *mut c_void, tag_id: u32) {
    match tag_id {
        0 => {
            let slice = CStr::from_ptr(c_ptr as *const c_char);
            let string = slice.to_str().unwrap();
            eprintln!("Roc hit a panic: {}", string);
            std::process::exit(1);
        }
        _ => todo!(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn roc_memcpy(dst: *mut c_void, src: *mut c_void, n: usize) -> *mut c_void {
    libc::memcpy(dst, src, n)
}

#[no_mangle]
pub unsafe extern "C" fn roc_memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    libc::memset(dst, c, n)
}

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    plot();
    let exit_code = 0;
    exit_code
}

fn plot() {
    plot_().unwrap()
}

const OUT_FILE_PATH: &'static str = "./plot.png";
fn plot_() -> Result<(), Box<dyn std::error::Error>> {
    let options = get_options();
    let output_file_path = options.outputFilePath.as_str();
    let area = BitMapBackend::new(output_file_path, (1024, 768)).into_drawing_area();
    area.fill(&WHITE)?;
    let area = area.titled(options.title.as_str(), ("sans-serif", 60))?;
    area.present().expect(
        format!("I failed to draw your plot to {} !", output_file_path).as_str(),
    );
    println!("I drew your plot to {}", output_file_path);
    Ok(())
}

fn get_options() -> Options {
    let o;
    unsafe {
        o = roc_options();
    }
    o
}
