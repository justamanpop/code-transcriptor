use regex::{Regex, Captures};

use std::borrow::Cow;

use crate::string_utils;
use crate::utils;

#[cfg(test)]
mod tests;

pub fn clean_transcript(mut transcript: String) -> String {
    transcript = lowercase_go_keywords(transcript);
    transcript = replace_special_chars(transcript);
    utils::log(
        "special chars replaced",
        transcript.clone(),
    );
    transcript = add_newline_after_assignments(transcript);
    utils::log(
        "newlines added",
        transcript.clone(),
    );
    transcript = edge_case_replacements(transcript);

    let mut transcript_lines = string_utils::split(transcript);
    utils::log(
        "lines with keywords lowercased and special chars replaced",
        transcript_lines.clone(),
    );

    transcript_lines = add_curly_braces(transcript_lines);
    transcript_lines.join("\n")
}

/// Ensures language keywords like type, interface, if, etc. are never capitalized.
/// TODO: make this work for all cases
fn lowercase_go_keywords(transcript: String) -> String {
    let re = Regex::new(r"(?i)(if )|(for )|(type )|( interface)|( struct)|( true)| ( false)").unwrap();
    re.replace_all(&transcript, |caps: &Captures| {
        caps[0].to_lowercase()
    }).to_string()
}

/// Replaces words for special characters equals with their literal symbol
/// E.g   colon with :, equals with =, etc.
fn replace_special_chars(transcript: String) -> String {
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
        r"(?P<dq>double quotes)|",
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

fn add_newline_after_assignments(transcript: String) -> String {
    let re = Regex::new(
        concat!(
        r"(?i)",
         
        r"(?P<true>:= true)|",
        r"(?P<false>:= false)|",
        //starts with :=, then the expression (?:".*?"\+) 0 or more times [it means anything inside double quotes followed by a +], then
        // the same expression once. Meant to capture string, or string + string + string
        r#"(?P<string>:= (?:".*?"\+)*".*?")"#,
        )
    ).unwrap();
    re.replace_all(&transcript, |caps: &Captures| format!("{}\n", &caps[0]))
        .to_string()
}

fn edge_case_replacements(transcript: String) -> String {
    let re = Regex::new(concat!(
        r"(?i)",
        r#"(?P<dq_trailing_and_leading_space>"\s.+?\s")"#,
    )).unwrap();
    re.replace_all(&transcript, |caps: &Captures| {
        if let Some(m) = caps.name("dq_trailing_and_leading_space") {
            let full_match = m.as_str();
            let trimmed_inner = full_match
                .trim_matches('"')
                .trim();
            if trimmed_inner == "space" {
                return format!("\" \"");
            }
            return format!("\"{}\"", trimmed_inner);
        }
        if let Some(_) = caps.name("dq_only_space") {
            return "\" \"".to_string()
        }
        caps[0].to_string()
    }).to_string()
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
