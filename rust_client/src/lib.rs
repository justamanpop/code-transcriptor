extern crate libc;

use std::io::Write;
use std::os::unix::net::{UnixStream};
use std::ffi::{CStr, CString};
use libc::c_char;

#[no_mangle]
pub extern "C" fn transcribe_audio(audio_file_path_ptr: *const libc::c_char, socket_file_path_ptr: *const libc::c_char) -> *mut c_char {
    let socket_file_path_str = unsafe {
        CStr::from_ptr(socket_file_path_ptr).to_str().unwrap()
    };

    let audio_file_path_str = unsafe {
        CStr::from_ptr(audio_file_path_ptr).to_str().unwrap()
    };

    let mut stream = UnixStream::connect(socket_file_path_str).unwrap();
    stream.write_all(audio_file_path_str.as_bytes());
    CString::new(socket_file_path_str).unwrap().into_raw()
}
