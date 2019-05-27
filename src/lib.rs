extern crate boxcars;
extern crate failure;

use std::fs::File;
use std::io::{Read};
use std::os::raw::c_char;
use std::ffi::CStr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[repr(C)]
pub struct ReplayObj {
    pub header_size: i32,
    pub header_crc: i32,
    pub major_version: i32,
    pub minor_version: i32
}


#[no_mangle]
pub extern fn print_text(text: *const c_char) {
    assert!(!text.is_null());
    let c_str = unsafe { CStr::from_ptr(text) };
    let string = c_str.to_str().expect("Not a valid UTF-8 string");
    println!("{}", string);
}

#[no_mangle]
pub extern fn parse(path: *const c_char) -> *mut ReplayObj {
    let null = &mut ReplayObj {
            header_size: 0,
            header_crc: 0,
            major_version: 0,
            minor_version: 0
    };
    println!("Hello World!");
    assert!(!path.is_null());
    println!("Not null!");
    let c_str = unsafe { CStr::from_ptr(path) };
    println!("null!");
    println!("{:?}", c_str.to_bytes());
    let string = c_str.to_str().expect("Invalid UTF-8 string");
    println!("{}", string);
    if let Ok(string) = c_str.to_str() {
        print!("All good");
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
            minor_version: replay.minor_version
        });
        Box::into_raw(rrr);
    }
    return null
}

#[no_mangle]
pub extern "C" fn dub(x: i32) -> i32 {
    x * 2
}