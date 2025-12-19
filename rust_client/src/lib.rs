extern crate libc;
use libc::c_char;
extern crate regex;

use std::string::String;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::ffi::{CStr, CString};

mod go;
mod string_utils;
mod utils;

#[no_mangle]
pub extern "C" fn transcribe_audio(
    audio_file_path_ptr: *const libc::c_char,
    socket_file_path_ptr: *const libc::c_char,
    filetype_ptr: *const libc::c_char,
) -> *mut c_char {
    utils::delete_file();
    let socket_file_path_str = unsafe { CStr::from_ptr(socket_file_path_ptr).to_str().unwrap() };

    let audio_file_path_str = unsafe { CStr::from_ptr(audio_file_path_ptr).to_str().unwrap() };

    let filetype_str = unsafe { CStr::from_ptr(filetype_ptr).to_str().unwrap() };

    utils::log(
        "data sent over socket to python daemon",
        format!(
            "socket path {}, audio file path {}, filetype {}",
            socket_file_path_str,
            audio_file_path_str,
            filetype_str
        ),
    );

    let transcript = get_transcript(audio_file_path_str, socket_file_path_str, filetype_str);
    let cleaned_transcript = clean_transcript(transcript, filetype_str);

    CString::new(cleaned_transcript).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        #[allow(unused_must_use)]
        CString::from_raw(s);
    }
}

/// Writes audio file path and filetype to UNIX socket that transcription daemon listens on.
/// Daemon responds with transcription on the same socket, this function reads and returns that.
fn get_transcript(audio_file_path: &str, socket_file_path: &str, filetype: &str) -> String {
    let mut stream = UnixStream::connect(socket_file_path).unwrap();

    let message = format!("{}x-x-x{}", audio_file_path, filetype);
    stream.write_all(message.as_bytes()).expect(
        "unable to write to UNIX socket",
    );

    let mut transcript = String::new();
    stream.read_to_string(&mut transcript).unwrap();
    utils::log("transcript returned from daemon", transcript.clone());
    return transcript;
}

/// The raw transcript is not valid code, it contains punctuation symbols, capitalizations, no braces
/// and parantheses, etc.
///
/// This function cleans up all of that and makes output valid code. Where possible it infers where
/// curly braces, newlines or other symbols are needed and inserts them, saving the speaker from
/// having to repeatedly specify those.
fn clean_transcript(mut transcript: String, filetype: &str) -> String {
    transcript = string_utils::strip_punctuation(transcript);
    utils::log("punctuation stripped", transcript.clone());
    match filetype {
        "go" => {
            go::clean_transcript(transcript)
        }
        _ => transcript,
    }
}
