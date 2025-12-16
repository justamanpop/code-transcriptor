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
use std::fmt::Debug;


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

    log(
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
        CString::from_raw(s);
    }
}

fn get_transcript(audio_file_path: &str, socket_file_path: &str, filetype: &str) -> String {
    let mut stream = UnixStream::connect(socket_file_path).unwrap();

    let message = format!("{}x-x-x{}", audio_file_path, filetype);
    stream.write_all(message.as_bytes()).expect(
        "unable to write to UNIX socket",
    );

    let mut transcript = String::new();
    stream.read_to_string(&mut transcript).unwrap();
    log("transcript returned from daemon", transcript.clone());
    return transcript;
}

fn clean_transcript(mut transcript: String, filetype: &str) -> String {
    transcript = strip_punctuation(transcript);
    log("punctuation stripped", transcript.clone());
    match filetype {
        "go" => {
            transcript = lowercase_go_keywords(transcript);
            transcript = replace_go_special_chars(transcript);

            let mut transcript_lines = split_into_lines(transcript);
            log(
                "lines with keywords lowercased and special chars replaced",
                transcript_lines.clone(),
            );

            transcript_lines = add_curly_braces(transcript_lines);
            transcript_lines.join("\n")
        }
        _ => transcript,
    }
}

fn add_curly_braces(transcript_lines: Vec<String>) -> Vec<String> {
    transcript_lines
        .into_iter()
        .map(|mut line| {
            if (line).starts_with("if ") || (line).starts_with("type ") {
                line.push_str(" {")
            } 
            line
        })
        .collect()
}

fn strip_punctuation(transcript: String) -> String {
    let re = Regex::new(r"\p{P}").unwrap();
    re.replace_all(&transcript, "").trim().to_string()
}

fn lowercase_go_keywords(transcript: String) -> String {
    let re = Regex::new(r"(?i)(if)|(for)|(type)|(interface)|(struct)").unwrap();
    re.replace_all(&transcript, |caps: &Captures| if caps.get(1).is_some() {
        Cow::Borrowed("if")
    } else if caps.get(2).is_some() {
        Cow::Borrowed("for")
    } else if caps.get(3).is_some() {
        Cow::Borrowed("type")
    } else if caps.get(4).is_some() {
        Cow::Borrowed("interface")
    } else if caps.get(5).is_some() {
        Cow::Borrowed("struct")
    } else {
        Cow::Borrowed(transcript.as_str())
    }).to_string()
}

fn replace_go_special_chars(transcript: String) -> String {
    let re = Regex::new(
        r"(?i)(colon equals)|(equals equals)|(equals)|(colon)|(close brackets)|(brackets)|(newline|new line)"
    ).unwrap();
    re.replace_all(&transcript, |caps: &Captures| if caps.get(1).is_some() {
        Cow::Borrowed(":=")
    } else if caps.get(2).is_some() {
        Cow::Borrowed("==")
    } else if caps.get(3).is_some() {
        Cow::Borrowed("=")
    } else if caps.get(4).is_some() {
        Cow::Borrowed(":")
    } else if caps.get(5).is_some() {
        Cow::Borrowed(")")
    } else if caps.get(6).is_some() {
        Cow::Borrowed("(")
    } else if caps.get(7).is_some() {
        Cow::Borrowed("\n")
    } else {
        Cow::Borrowed(transcript.as_str())
    }).to_string()
}

fn split_into_lines(transcript: String) -> Vec<String> {
    transcript.split("\n").map(str::to_string).collect()
}


fn log<T>(prefix: &str, data: T)
where
    T: Debug,
{
    let log_string = format!("{} {:?}\n", prefix, data);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/home/anishs/development/voice_to_code/log.txt")
        .unwrap();
    file.write_all(log_string.as_bytes()).expect(
        "could not write to log file",
    );
}

fn delete_file() {
    let _ = remove_file("/home/anishs/development/voice_to_code/log.txt");
    ()
}
