extern crate libc;
use libc::c_char;
extern crate regex;
use regex::{Regex, Captures};

use std::string::String;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::ffi::{CStr, CString};
use std::fs::{OpenOptions, remove_file};
use std::borrow::Cow;


#[no_mangle]
pub extern "C" fn transcribe_audio(
    audio_file_path_ptr: *const libc::c_char,
    socket_file_path_ptr: *const libc::c_char,
    filetype_ptr: *const libc::c_char,
) -> *mut c_char {
    delete_file();
    let socket_file_path_str = unsafe { CStr::from_ptr(socket_file_path_ptr).to_str().unwrap() };

    let audio_file_path_str = unsafe { CStr::from_ptr(audio_file_path_ptr).to_str().unwrap() };

    let filetype_str = unsafe { CStr::from_ptr(filetype_ptr).to_str().unwrap() };

    append_string_to_file(format!(
        "socket path {}, audio file path {}, filetype {}",
        socket_file_path_str,
        audio_file_path_str,
        filetype_str
    ));

    let transcript = get_transcript(audio_file_path_str, socket_file_path_str);
    let cleaned_transcript = clean_transcript(transcript, filetype_str);

    CString::new(cleaned_transcript).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s);
    }
}

fn get_transcript(audio_file_path: &str, socket_file_path: &str) -> String {
    let mut stream = UnixStream::connect(socket_file_path).unwrap();

    stream.write_all(audio_file_path.as_bytes()).expect(
        "unable to write to UNIX socket",
    );

    let mut transcript = String::new();
    stream.read_to_string(&mut transcript).unwrap();
    return transcript;
}

fn clean_transcript(mut transcript: String, filetype: &str) -> String {
    append_string_to_file(transcript.clone());
    transcript = strip_punctuation(transcript);
    append_string_to_file(transcript.clone());
    match filetype {
        "go" => clean_go_transcript(transcript),
        _ => transcript,
    }
}

fn strip_punctuation(transcript: String) -> String {
    let re = Regex::new(r"\p{P}").unwrap();
    re.replace_all(&transcript, "").to_string()
}

fn clean_go_transcript(transcript: String) -> String {
    let re = Regex::new(r"(?i)(colon equals)|(equals equals)|(equals)|(colon)|(curly)|(close curly)|(new line)").unwrap();
    re.replace_all(&transcript, |caps: &Captures| if caps.get(1).is_some() {
        Cow::Borrowed(":=")
    } else if caps.get(2).is_some() {
        Cow::Borrowed("==")
    } else if caps.get(3).is_some() {
        Cow::Borrowed("=")

    } else if caps.get(4).is_some() {
        Cow::Borrowed(":")

    } else if caps.get(5).is_some() { 
       Cow::Borrowed("{")
    } else if caps.get(6).is_some() {
        Cow::Borrowed("}")
    
    } else if caps.get(7).is_some() {
        Cow::Borrowed("\n")
    } else {
        //should never happen
        Cow::Borrowed(transcript.as_str())
    }).to_string()
}

fn append_string_to_file(mut s: String) {
    s.push_str("\n");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/home/anishs/development/voice_to_code/log.txt")
        .unwrap();
    file.write_all(s.as_bytes()).expect(
        "could not write to log file",
    );
}

fn delete_file() {
    remove_file("/home/anishs/development/voice_to_code/log.txt").expect("failed to delete log file");
    ()
}
