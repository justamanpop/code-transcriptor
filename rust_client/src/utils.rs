use std::fs::{OpenOptions, remove_file};
use std::fmt::Debug;

use std::io::{Write};

pub fn log<T>(prefix: &str, data: T)
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

pub fn delete_file() {
    let _ = remove_file("/home/anishs/development/voice_to_code/log.txt");
    ()
}
