extern crate boxcars;
extern crate failure;
extern crate libc;

use std::ffi::{CStr};
use libc::{c_void, size_t};
use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::{mem, slice};


#[repr(C)]
pub struct ReplayObj {
    pub header_size: i32,
    pub header_crc: i32,
    pub major_version: i32,
    pub minor_version: i32,
    pub arr: Array,
}

/// Wrapper for a void pointer to a sequence of 2-element arrays representing points, and the sequence length. Used for FFI.
#[repr(C)]
pub struct Array {
    pub data: *const c_void,
    pub len: size_t,
}
fn reconstitute(arr: &Array) -> Vec<[f64; 2]> {
    unsafe { slice::from_raw_parts(arr.data as *mut [f64; 2], arr.len).to_vec() }
}
fn gen_array(v: Vec<[f64; 2]>) -> *const Array {
    let array = Array {
        data: v.as_ptr() as *const c_void,
        len: v.len() as size_t,
    };
    mem::forget(v);
    &array
}
fn gen_array1d(v: Vec<f64>) -> Array {
    let array = Array {
        data: v.as_ptr() as *const c_void,
        len: v.len() as size_t,
    };
    mem::forget(v);
    array
}
#[no_mangle]
pub extern fn parse(text: *const c_char) -> *mut ReplayObj {
    assert!(!text.is_null());
    let c_str = unsafe { CStr::from_ptr(text) };
    if let Ok(string) = c_str.to_str() {
        println!("Parsing {}", string);
        let filename = string;
        let mut f = File::open(filename).unwrap();
        let mut buffer = vec![];
        f.read_to_end(&mut buffer).unwrap();
        let replay = boxcars::ParserBuilder::new(&buffer)
            .on_error_check_crc()
            .parse()
            .unwrap();
        let ext_vec = vec![
            [4.0, 1.0],
            [5.0, 2.0],
            [5.0, 3.0],
            [4.0, 4.0],
            [3.0, 4.0],
            [2.0, 3.0],
            [2.0, 2.0],
            [3.0, 1.0],
            [4.0, 1.0],
        ];

        let d1_vec = vec![
            1.0, 2.0, 3.0
        ];
        let rrr = Box::new(ReplayObj {
            header_size: replay.header_size,
            header_crc: replay.header_crc,
            major_version: replay.major_version,
            minor_version: replay.minor_version,
            arr: gen_array1d(d1_vec)
        });
        println!("Rust Major Version {}", replay.major_version);
//        for property in replay.properties {
//            println!("{}", property.0);
//            if let HeaderProp::Str(v) = property.1 {
//                println!("{}", v);
//            }
//            else if let HeaderProp::Int(v) = property.1 {
//                println!("{}", v);
//            }
//        }
        Box::into_raw(rrr)
    } else {
        return null_mut();
    }
}