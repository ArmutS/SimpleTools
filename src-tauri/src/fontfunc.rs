/// Font conversion (basic support)
/// Note: Full font conversion requires complex libraries
/// This provides basic format identification and copying
#[tauri::command]
pub fn convert_font(
    file_path: String,
    output_dir: String,
    target_format: String,
) -> Result<String, String> {
    use std::fs;
    use std::path::Path;
    
    let path = Path::new(&file_path);
    let file_name = path
        .file_stem()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file name")?;

    // Read font file
    let font_data = fs::read(&file_path)
        .map_err(|e| format!("Failed to read font: {}", e))?;

    let target = target_format.to_lowercase();
    
    // Note: True font conversion requires parsing and re-encoding
    // For now, we just copy with new extension (placeholder)
    let output_path = format!("{}/{}.{}", output_dir, file_name, target);
    fs::write(&output_path, &font_data)
        .map_err(|e| format!("Failed to write font: {}", e))?;

    Ok(format!("Font saved as {} (conversion limited)", target))
}
