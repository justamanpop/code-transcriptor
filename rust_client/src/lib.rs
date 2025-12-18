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
    log("transcript returned from daemon", transcript.clone());
    return transcript;
}

/// The raw transcript is not valid code, it contains punctuation symbols, capitalizations, no braces
/// and parantheses, etc.
///
/// This function cleans up all of that and makes output valid code. Where possible it infers where
/// curly braces, newlines or other symbols are needed and inserts them, saving the speaker from
/// having to repeatedly specify those.
fn clean_transcript(mut transcript: String, filetype: &str) -> String {
    transcript = strip_punctuation(transcript);
    log("punctuation stripped", transcript.clone());
    match filetype {
        "go" => {
            transcript = go_replace_special_chars(transcript);
            transcript = go_add_newline_after_assignments(transcript);
            transcript = go_edge_case_replacements(transcript);
            transcript = go_lowercase_go_keywords(transcript);

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

fn go_edge_case_replacements(transcript: String) -> String {
    let re = Regex::new(concat!(
        r"(?i)",
        r#"(?P<dq_space>"space")"#,
    )).unwrap();
    re.replace_all(&transcript, |caps: &Captures| {
        if caps.name("dq_space").is_some() {
            return Cow::Borrowed("\" \"");
        }
        Cow::Borrowed(transcript.as_str())
    }).to_string()
}

/// Strip all punctuation except periods, since decimal numbers are output with those.
///
/// To identify and strip periods used as full stops, finding and removing period space.
///
/// Finally, if period is the last char of the transcript, there is no space after, it won't be caught by above cases,
/// so stripping it as suffix.
fn strip_punctuation(transcript: String) -> String {
    let re = Regex::new(r"(?P<non_period_punc>[\p{P}--.])|(?P<period_space>\. )").unwrap();
    let replaced = re.replace_all(&transcript, |caps: &Captures| {
        if caps.name("period_space").is_some() {
            return Cow::Borrowed(" ");
        } else if caps.name("non_period_punc").is_some() {
            return Cow::Borrowed("");
        }
        Cow::Borrowed(transcript.as_str())
    });
    let trimmed = replaced.trim();
    trimmed
        .strip_suffix(".")
        .map(|s| s.to_string())
        .unwrap_or_else(|| trimmed.to_string())
}

/// Ensures language keywords like type, interface, if, etc. are never capitalized.
fn go_lowercase_go_keywords(transcript: String) -> String {
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

/// Replaces words for special characters equals with their literal symbol
/// E.g   colon with :, equals with =, etc.
fn go_replace_special_chars(transcript: String) -> String {
    let re = Regex::new(concat!(
        r"(?i)",
        r"(?P<ce>colon equals)|",
        r"(?P<ee>equals equals)|",
        r"(?P<eq>equals)|",
        r"(?P<co>colon)|",
        r"(?P<cb>close brackets)|",
        r"(?P<br>brackets)|",
        r"(?P<pl>plus)|",
        r"(?P<mi>minus)|",
        r"(?P<dq>\s*double quotes\s*)|",
        r"(?P<nl>newline|new line)",
    )).unwrap();
    re.replace_all(&transcript, |caps: &Captures| {
        if caps.name("ce").is_some() {
            return Cow::Borrowed(":=");
        }
        if caps.name("ee").is_some() {
            return Cow::Borrowed("==");
        }
        if caps.name("eq").is_some() {
            return Cow::Borrowed("=");
        }
        if caps.name("co").is_some() {
            return Cow::Borrowed(":");
        }
        if caps.name("cb").is_some() {
            return Cow::Borrowed(")");
        }
        if caps.name("br").is_some() {
            return Cow::Borrowed("(");
        }
        if caps.name("pl").is_some() {
            return Cow::Borrowed("+");
        }
        if caps.name("mi").is_some() {
            return Cow::Borrowed("-");
        }
        if caps.name("dq").is_some() {
            return Cow::Borrowed("\"");
        }
        if caps.name("nl").is_some() {
            return Cow::Borrowed("\n");
        }

        Cow::Borrowed(transcript.as_str())
    }).to_string()
}

fn go_add_newline_after_assignments(transcript: String) -> String {
    let re = Regex::new(
        r#"(?i)(?P<true>:= true)|(?P<false>:= false)|(?P<string>(?:".*?"\+)+".*?")"#,
    ).unwrap();
    re.replace_all(&transcript, |caps: &Captures| format!("{}\n", &caps[0]))
        .to_string()
}

fn split_into_lines(transcript: String) -> Vec<String> {
    transcript.split("\n").map(str::to_string).collect()
}

fn add_curly_braces(transcript_lines: Vec<String>) -> Vec<String> {
    transcript_lines
        .into_iter()
        .map(|mut line| {
            if ((line).starts_with("if ") || (line).starts_with(" if ") ||
                    (line).starts_with("type ") || (line).starts_with(" type ")) &&
                !(line.ends_with("{") || line.ends_with("{ "))
            {
                line.push_str(" {")
            }
            line
        })
        .collect()
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
