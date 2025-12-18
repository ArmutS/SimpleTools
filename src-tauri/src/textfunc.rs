use similar::{ChangeTag, TextDiff};
use serde::Serialize;

#[derive(Serialize)]
pub struct DiffResult {
    pub text: String,
    pub tag: String
}

#[tauri::command(rename_all = "snake_case")]
pub fn process_text_diff(left_in: String, right_in: String, select: i32) -> Vec<DiffResult>{

        let mut result_vec = Vec::new();

        let diff = match select{
            0 => TextDiff::from_chars(&left_in, &right_in),
            1 => TextDiff::from_words(&left_in, &right_in),
            2 => TextDiff::from_lines(&left_in, &right_in),
            _ => TextDiff::from_words(&left_in, &right_in)
        };

        for dif_text in diff.iter_all_changes(){

            let tag = match dif_text.tag() {
                ChangeTag::Delete => "left",
                ChangeTag::Insert => "right",
                _ => "not"
            };


            result_vec.push(DiffResult { text: dif_text.to_string(), tag: tag.to_string() });
        }
        result_vec
}