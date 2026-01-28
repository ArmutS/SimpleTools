use regex::Regex;
use std::fs;
use std::path::Path;

/// Convert subtitle files between formats (SRT, VTT, ASS, SSA, SUB)
#[tauri::command]
pub fn convert_subtitle(
    file_path: String,
    output_dir: String,
    target_format: String,
) -> Result<String, String> {
    let path = Path::new(&file_path);
    let file_name = path
        .file_stem()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file name")?;
    
    let input_ext = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or("No file extension")?
        .to_lowercase();

    // Read input file
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse to intermediate format
    let subtitles = match input_ext.as_str() {
        "srt" => parse_srt(&content)?,
        "vtt" => parse_vtt(&content)?,
        "ass" | "ssa" => parse_ass(&content)?,
        _ => return Err(format!("Unsupported input format: {}", input_ext)),
    };

    // Convert to target format
    let target = target_format.to_lowercase();
    let output_content = match target.as_str() {
        "srt" => generate_srt(&subtitles),
        "vtt" => generate_vtt(&subtitles),
        "ass" => generate_ass(&subtitles),
        "ssa" => generate_ssa(&subtitles),
        _ => return Err(format!("Unsupported output format: {}", target)),
    };

    // Write output file
    let output_path = format!("{}/{}.{}", output_dir, file_name, target);
    fs::write(&output_path, output_content)
        .map_err(|e| format!("Failed to write output: {}", e))?;

    Ok(format!("Converted {} to {}", file_name, target))
}

#[derive(Debug, Clone)]
struct Subtitle {
    start_time: String,
    end_time: String,
    text: String,
}

// SRT Parser
fn parse_srt(content: &str) -> Result<Vec<Subtitle>, String> {
    let mut subtitles = Vec::new();
    let blocks: Vec<&str> = content.split("\n\n").collect();

    for block in blocks {
        let lines: Vec<&str> = block.trim().lines().collect();
        if lines.len() < 3 {
            continue;
        }

        // Line 0: index, Line 1: timestamps, Line 2+: text
        let timestamps = lines[1];
        let time_parts: Vec<&str> = timestamps.split(" --> ").collect();
        if time_parts.len() != 2 {
            continue;
        }

        let text = lines[2..].join("\n");
        subtitles.push(Subtitle {
            start_time: srt_to_standard_time(time_parts[0]),
            end_time: srt_to_standard_time(time_parts[1]),
            text,
        });
    }

    Ok(subtitles)
}

// VTT Parser
fn parse_vtt(content: &str) -> Result<Vec<Subtitle>, String> {
    let content = content.trim_start_matches("WEBVTT\n").trim_start_matches("WEBVTT\r\n");
    let mut subtitles = Vec::new();
    let blocks: Vec<&str> = content.split("\n\n").collect();

    for block in blocks {
        let lines: Vec<&str> = block.trim().lines().collect();
        if lines.is_empty() {
            continue;
        }

        // Find timestamp line
        let mut timestamp_idx = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.contains("-->") {
                timestamp_idx = i;
                break;
            }
        }

        if timestamp_idx < lines.len() {
            let timestamps = lines[timestamp_idx];
            let time_parts: Vec<&str> = timestamps.split(" --> ").collect();
            if time_parts.len() != 2 {
                continue;
            }

            let text = lines[timestamp_idx + 1..].join("\n");
            subtitles.push(Subtitle {
                start_time: vtt_to_standard_time(time_parts[0]),
                end_time: vtt_to_standard_time(time_parts[1]),
                text,
            });
        }
    }

    Ok(subtitles)
}

// ASS/SSA Parser (simplified - extracts dialogue lines)
fn parse_ass(content: &str) -> Result<Vec<Subtitle>, String> {
    let mut subtitles = Vec::new();
    
    for line in content.lines() {
        if line.starts_with("Dialogue:") {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 10 {
                // ASS format: Dialogue: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
                let start = parts[1].trim();
                let end = parts[2].trim();
                let text = parts[9..].join(",").replace("\\N", "\n");

                subtitles.push(Subtitle {
                    start_time: ass_to_standard_time(start),
                    end_time: ass_to_standard_time(end),
                    text,
                });
            }
        }
    }

    Ok(subtitles)
}

// SRT Generator
fn generate_srt(subtitles: &[Subtitle]) -> String {
    let mut output = String::new();
    for (i, sub) in subtitles.iter().enumerate() {
        output.push_str(&format!("{}\n", i + 1));
        output.push_str(&format!(
            "{} --> {}\n",
            standard_to_srt_time(&sub.start_time),
            standard_to_srt_time(&sub.end_time)
        ));
        output.push_str(&sub.text);
        output.push_str("\n\n");
    }
    output
}

// VTT Generator
fn generate_vtt(subtitles: &[Subtitle]) -> String {
    let mut output = String::from("WEBVTT\n\n");
    for sub in subtitles {
        output.push_str(&format!(
            "{} --> {}\n",
            standard_to_vtt_time(&sub.start_time),
            standard_to_vtt_time(&sub.end_time)
        ));
        output.push_str(&sub.text);
        output.push_str("\n\n");
    }
    output
}

// ASS Generator
fn generate_ass(subtitles: &[Subtitle]) -> String {
    let mut output = String::from("[Script Info]\nTitle: Converted Subtitle\nScriptType: v4.00+\n\n[V4+ Styles]\nFormat: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding\nStyle: Default,Arial,20,&H00FFFFFF,&H000088EF,&H00000000,&H00666666,-1,0,0,0,100,100,0,0,1,2,0,2,10,10,10,1\n\n[Events]\nFormat: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n");
    
    for sub in subtitles {
        let text = sub.text.replace('\n', "\\N");
        output.push_str(&format!(
            "Dialogue: 0,{},{},Default,,0,0,0,,{}\n",
            standard_to_ass_time(&sub.start_time),
            standard_to_ass_time(&sub.end_time),
            text
        ));
    }
    output
}

// SSA Generator (similar to ASS but v4.00)
fn generate_ssa(subtitles: &[Subtitle]) -> String {
    let mut output = String::from("[Script Info]\nTitle: Converted Subtitle\n\n[V4 Styles]\nFormat: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, TertiaryColour, BackColour, Bold, Italic, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, AlphaLevel, Encoding\nStyle: Default,Arial,20,16777215,65535,65535,0,-1,0,1,3,0,2,30,30,30,0,0\n\n[Events]\nFormat: Marked, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n");
    
    for sub in subtitles {
        let text = sub.text.replace('\n', "\\N");
        output.push_str(&format!(
            "Dialogue: Marked=0,{},{},Default,,0,0,0,,{}\n",
            standard_to_ass_time(&sub.start_time),
            standard_to_ass_time(&sub.end_time),
            text
        ));
    }
    output
}

// Time conversion helpers
fn srt_to_standard_time(time: &str) -> String {
    time.replace(',', ".")
}

fn vtt_to_standard_time(time: &str) -> String {
    time.to_string()
}

fn ass_to_standard_time(time: &str) -> String {
    // ASS format: 0:00:00.00 -> Standard: 00:00:00.000
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() == 3 {
        let h = parts[0].parse::<u32>().unwrap_or(0);
        let m = parts[1].parse::<u32>().unwrap_or(0);
        let s_parts: Vec<&str> = parts[2].split('.').collect();
        let s = s_parts[0].parse::<u32>().unwrap_or(0);
        let ms = if s_parts.len() > 1 {
            format!("{:0<3}", s_parts[1])
        } else {
            "000".to_string()
        };
        format!("{:02}:{:02}:{:02}.{}", h, m, s, ms)
    } else {
        time.to_string()
    }
}

fn standard_to_srt_time(time: &str) -> String {
    time.replace('.', ",")
}

fn standard_to_vtt_time(time: &str) -> String {
    time.to_string()
}

fn standard_to_ass_time(time: &str) -> String {
    // Standard: 00:00:00.000 -> ASS: 0:00:00.00
    time.trim_start_matches('0')
        .trim_start_matches(':')
        .replacen('.', ".", 1)
        .chars()
        .take(time.len() - 1) // Remove last digit of milliseconds
        .collect()
}
