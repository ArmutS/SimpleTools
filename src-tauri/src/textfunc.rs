use std::collections::HashSet;

use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};

#[derive(Serialize)]
pub struct DiffResult {
    pub text: String,
    pub tag: String,
}

#[tauri::command(rename_all = "snake_case")]
pub fn process_text_diff(left_in: String, right_in: String, select: i32) -> Vec<DiffResult> {
    let mut result_vec = Vec::new();

    let diff = match select {
        0 => TextDiff::from_chars(&left_in, &right_in),
        1 => TextDiff::from_words(&left_in, &right_in),
        2 => TextDiff::from_lines(&left_in, &right_in),
        _ => TextDiff::from_words(&left_in, &right_in),
    };

    for dif_text in diff.iter_all_changes() {
        let tag = match dif_text.tag() {
            ChangeTag::Delete => "left",
            ChangeTag::Insert => "right",
            _ => "not",
        };

        result_vec.push(DiffResult {
            text: dif_text.to_string(),
            tag: tag.to_string(),
        });
    }
    result_vec
}

#[derive(Serialize)]
pub struct MatchResults {
    text: String,
    start: usize,
    end: usize,
}

#[tauri::command(rename_all = "snake_case")]
pub fn process_text_reg(
    current_regex: &str,
    current_text: &str,
    current_flags: &str,
) -> Result<Vec<MatchResults>, String> {
    let mut build_reg = RegexBuilder::new(current_regex);

    if current_flags.contains("s") {
        build_reg.dot_matches_new_line(true);
    }

    if current_flags.contains("i") {
        build_reg.case_insensitive(true);
    }

    if current_flags.contains("m") {
        build_reg.multi_line(true);
    }

    let process_regex = build_reg.build().map_err(|e| e.to_string())?;

    Ok(process_regex
        .find_iter(current_text)
        .map(|reg| MatchResults {
            text: reg.as_str().to_string(),
            start: reg.start(),
            end: reg.end(),
        })
        .collect())
}

#[derive(Deserialize)]
pub struct Options {
    email: bool,
    url: bool,
    ip: bool,
    hashtag: bool,
    log_error: bool,
}

#[tauri::command(rename_all = "snake_case")]
pub fn process_extractor(current_text: &str, options: Options) -> Result<Vec<String>, String> {
    let mut result: HashSet<String> = HashSet::new();
    let patterns = [
        (options.email, r"(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}"),
        (
            options.url,
            r"(?i)https?://(?:localhost|127\.0\.0\.1|(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6})\b(?::\d+)?(?:[-a-zA-Z0-9()@:%_\+.~#?&//=]*)",
        ),
        (options.ip, r"(?:[0-9]{1,3}\.){3}[0-9]{1,3}"),
        (
            options.ip,
            r"(?i)(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:))",
        ),
        (options.hashtag, r"#[a-zA-Z0-9_]+"),
        (
            options.log_error,
            r"(?im)^.*(error|err|fatal|critical|exception|fail|warning).*$",
        ),
    ];

    for (option, pattern) in patterns {
        if option {
            let regex = Regex::new(pattern).map_err(|e| e.to_string())?;

            result.extend(
                regex
                    .find_iter(current_text)
                    .map(|reg| reg.as_str().to_string()),
            );
        }
    }
    Ok(result.into_iter().collect())
}
