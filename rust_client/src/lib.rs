extern crate libc;

use std::string::String;
use std::io::{Read, Write};
use std::os::unix::net::{UnixStream};
use std::ffi::{CStr, CString};
use libc::c_char;

#[no_mangle]
pub extern "C" fn transcribe_audio(audio_file_path_ptr: *const libc::c_char, socket_file_path_ptr: *const libc::c_char, filetype_ptr: *const libc::c_char) -> *mut c_char {
    let socket_file_path_str = unsafe {
        CStr::from_ptr(socket_file_path_ptr).to_str().unwrap()
    };

    let audio_file_path_str = unsafe {
        CStr::from_ptr(audio_file_path_ptr).to_str().unwrap()
    };

    let filetype_str = unsafe {
        CStr::from_ptr(filetype_ptr).to_str().unwrap()
    };

    let transcript = get_transcript(audio_file_path_str, socket_file_path_str);
    let cleaned_transcript = clean_transcript(transcript, filetype_str);
    CString::new(cleaned_transcript).unwrap().into_raw()
}

fn get_transcript(audio_file_path: &str, socket_file_path: &str)-> String {
    let mut stream = UnixStream::connect(socket_file_path).unwrap();

    stream.write_all(audio_file_path.as_bytes());

    let mut transcript = String::new();
    stream.read_to_string(&mut transcript).unwrap();
    return transcript
}

fn clean_transcript(mut transcript: String, filetype: &str) -> String {
    match filetype {
        "lua" => {
            transcript.push_str(" lua");
            transcript
        },
        _ => transcript
    }
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() { return; }
        CString::from_raw(s); 
    }
}
