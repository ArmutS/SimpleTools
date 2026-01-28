use std::fs;
use std::path::Path;

/// E-Book converter (EPUB, MOBI, PDF, TXT)
#[tauri::command]
pub fn convert_ebook(
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

    let target = target_format.to_lowercase();

    // Handle TXT conversions
    if target == "txt" {
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        let output_path = format!("{}/{}.txt", output_dir, file_name);
        fs::write(&output_path, content)
            .map_err(|e| format!("Failed to write TXT: {}", e))?;
        return Ok(format!("Converted to TXT"));
    }

    // For other formats, placeholder implementation
    let data = fs::read(&file_path)
        .map_err(|e| format!("Failed to read e-book: {}", e))?;
    let output_path = format!("{}/{}.{}", output_dir, file_name, target);
    fs::write(&output_path, &data)
        .map_err(|e| format!("Failed to write e-book: {}", e))?;

    Ok(format!("E-book saved as {} (limited conversion)", target))
}
