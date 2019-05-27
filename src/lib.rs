extern crate boxcars;
extern crate failure;
extern crate libc;

use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;
use std::ptr::null_mut;


#[repr(C)]
pub struct ReplayObj {
    pub header_size: i32,
    pub header_crc: i32,
    pub major_version: i32,
    pub minor_version: i32,
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
        let rrr = Box::new(ReplayObj {
            header_size: replay.header_size,
            header_crc: replay.header_crc,
            major_version: replay.major_version,
            minor_version: replay.minor_version,
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