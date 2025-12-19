use regex::{Regex, Captures};

use std::borrow::Cow;
/// Strip all punctuation except periods, since decimal numbers are output with those.
///
/// To identify and strip periods used as full stops, finding and removing period space.
///
/// Finally, if period is the last char of the transcript, there is no space after, it won't be caught by above cases,
/// so stripping it as suffix.
pub fn strip_punctuation(transcript: String) -> String {
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

pub fn split(transcript: String) -> Vec<String> {
    transcript.split("\n").map(str::to_string).collect()
}
