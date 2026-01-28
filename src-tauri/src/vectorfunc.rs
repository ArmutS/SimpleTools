use std::fs;
use std::path::Path;

/// Vector graphics converter (SVG, PDF, EPS)
#[tauri::command]
pub fn convert_vector(
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

    // SVG to SVG (just copy)
    if input_ext == "svg" && target == "svg" {
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read SVG: {}", e))?;
        let output_path = format!("{}/{}.svg", output_dir, file_name);
        fs::write(&output_path, content)
            .map_err(|e| format!("Failed to write SVG: {}", e))?;
        return Ok(format!("SVG copied"));
    }

    // Placeholder for other conversions
    let data = fs::read(&file_path)
        .map_err(|e| format!("Failed to read vector: {}", e))?;
    let output_path = format!("{}/{}.{}", output_dir, file_name, target);
    fs::write(&output_path, &data)
        .map_err(|e| format!("Failed to write vector: {}", e))?;

    Ok(format!("Vector saved as {} (limited conversion)", target))
}
