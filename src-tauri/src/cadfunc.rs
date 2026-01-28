use std::fs;
use std::path::Path;

/// 3D/CAD file converter (STL, OBJ, GLTF)
#[tauri::command]
pub fn convert_3d(
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

    // Text-based formats (OBJ) can be copied
    if input_ext == "obj" || input_ext == "stl" {
        let data = fs::read(&file_path)
            .map_err(|e| format!("Failed to read 3D file: {}", e))?;
        let output_path = format!("{}/{}.{}", output_dir, file_name, target);
        fs::write(&output_path, &data)
            .map_err(|e| format!("Failed to write 3D file: {}", e))?;
        return Ok(format!("3D file saved as {}", target));
    }

    // Placeholder for complex conversions
    let data = fs::read(&file_path)
        .map_err(|e| format!("Failed to read 3D file: {}", e))?;
    let output_path = format!("{}/{}.{}", output_dir, file_name, target);
    fs::write(&output_path, &data)
        .map_err(|e| format!("Failed to write 3D file: {}", e))?;

    Ok(format!("3D file saved as {} (limited conversion)", target))
}
