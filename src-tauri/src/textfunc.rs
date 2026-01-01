use base64::{engine::general_purpose, Engine as _};
use lipsum::lipsum;
use pulldown_cmark::{html, Options as MarkdownOptions, Parser};
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};
use slug::slugify;
use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

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

#[tauri::command(rename_all = "snake_case")]
pub fn process_strip(current_text: &str, pure_text: bool) -> Result<String, String> {
    let regex = Regex::new(r"<[^>]*>").map_err(|e| e.to_string())?;

    if pure_text {
        Ok(html_escape::decode_html_entities(&regex.replace_all(current_text, "")).into_owned())
    } else {
        Ok(regex.replace_all(current_text, "").into_owned())
    }
}

// --- NEW TOOLS ---

// 1. String Escaper / Unescaper
#[tauri::command(rename_all = "snake_case")]
pub fn process_string_escape(current_text: &str, mode: &str) -> String {
    if mode == "escape" {
        serde_json::to_string(current_text).unwrap_or_else(|_| "".to_string())
            .trim_matches('"') // serde adds quotes around the string, we usually want just the content escaped
            .to_string()
    } else {
        // Unescape: wrap in quotes and parse
        let wrapped = format!("\"{}\"", current_text);
        serde_json::from_str::<String>(&wrapped).unwrap_or_else(|_| "Error: Invalid format".to_string())
    }
}

// 2. Slug Generator
#[tauri::command(rename_all = "snake_case")]
pub fn process_slug_gen(current_text: &str) -> String {
    slugify(current_text)
}

// 3. JWT Decoder
#[derive(Serialize)]
pub struct JwtResult {
    header: String,
    payload: String,
}

#[tauri::command(rename_all = "snake_case")]
pub fn process_jwt_decode(token: &str) -> Result<JwtResult, String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid JWT structure (must have 3 parts).".to_string());
    }

    fn decode_part(part: &str) -> Result<String, String> {
         let bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(part)
            .or_else(|_| general_purpose::STANDARD_NO_PAD.decode(part))
            .or_else(|_| general_purpose::URL_SAFE.decode(part))
            .map_err(|e| format!("Base64 Error: {}", e))?;
        
        let s = String::from_utf8(bytes).map_err(|e| e.to_string())?;
        
        // Try to pretty print if it's JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
           Ok(serde_json::to_string_pretty(&json).unwrap_or(s))
        } else {
           Ok(s)
        }
    }

    Ok(JwtResult {
        header: decode_part(parts[0])?,
        payload: decode_part(parts[1])?,
    })
}

// 4. Markdown Preview
#[tauri::command(rename_all = "snake_case")]
pub fn process_markdown_preview(current_text: &str) -> String {
    let mut options = MarkdownOptions::empty();
    options.insert(MarkdownOptions::ENABLE_STRIKETHROUGH);
    options.insert(MarkdownOptions::ENABLE_TABLES);
    
    let parser = Parser::new_ext(current_text, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}


// 5. Lorem Ipsum
#[tauri::command(rename_all = "snake_case")]
pub fn process_lorem(count: usize, mode: &str) -> String {
    // lib lipsum generates words. 
    // We can use it to generate text.
    // However, it doesn't support "sentences" or "paragraphs" granularity perfectly out of box nicely without some logic,
    // but `lipsum(n)` generates n words roughly. 
    // Wait, `lipsum()` generates "Lorem ipsum..." string.
    // `lipsum(n)` generates around n bytes/words? No, it generates text.
    
    // Let's implement simple generation since `lipsum` crate is good for "lorem ipsum..."
    // actually `lipsum::lipsum(n)` generates n *words*.
    
    match mode {
        "words" => lipsum(count),
        "sentences" => {
            // Approximating sentences. 1 sentence ~ 8-15 words.
            let mut res = Vec::new();
            for _ in 0..count {
                 let s = lipsum(10); // generate ~10 words
                 // capitalize and add dot
                 let s = s.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default() + &s[1..] + ".";
                 res.push(s);
            }
            res.join(" ")
        },
        "paragraphs" | _ => {
             // Approximating paragraphs. 1 paragraph ~ 5 sentences ~ 50 words.
            let mut res = Vec::new();
            for _ in 0..count {
                // lipsum(50) roughly
                res.push(lipsum(50));
            }
            res.join("\n\n")
        }
    }
}


// 6. Text Obfuscator
#[tauri::command(rename_all = "snake_case")]
pub fn process_obfuscator(current_text: &str, mode: &str) -> String {
     match mode {
        "rot13" => {
             current_text.chars().map(|c| {
                match c {
                    'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
                    'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
                    _ => c,
                }
            }).collect()
        },
        "reverse" => current_text.chars().rev().collect(),
        "base64" => general_purpose::STANDARD.encode(current_text),
        _ => current_text.to_string(),
     }
}

// 7. Character Inspector
#[derive(Serialize)]
pub struct CharInfo {
    char: String,
    unicode: String,
    decimal: u32,
    entity: String,
}

#[tauri::command(rename_all = "snake_case")]
pub fn process_char_inspector(current_text: &str) -> Vec<CharInfo> {
    // Limit to 500 chars (graphemes) to match previous logic
    UnicodeSegmentation::graphemes(current_text, true).take(500).map(|c| {
        // c is a &str representing the grapheme cluster
        // We can get the first scalar value for "decimal" representation or sum them?
        // Usually inspector shows key data. If it's a multi-char grapheme, we might want to show info for the whole thing 
        // or just the first codepoint. The previous JS implementation did `codePointAt(0)`.
        // Let's iterate the chars inside the grapheme if needed, but for "decimal" code, 
        // usually the first scalar value is what people look for (or we loop).
        // Let's just output the first scalar value for simplicity similar to JS logic, 
        // but arguably we should show hex bytes for the whole sequence.
        // However, `c.chars().next()` gives the first char.
        
        let first_char = c.chars().next().unwrap_or('\0');
        let code = first_char as u32;

        CharInfo {
            char: c.to_string(),
            unicode: format!("U+{:04X}", code), // This technically only shows the first scalar. 
            // Better might be to show all scalars if multiple.
            // But let's stick to simple "first scalar" for now to match JS behavior `codePointAt(0)`.
            decimal: code,
            entity: format!("&#{};", code)
        }
    }).collect()
}
