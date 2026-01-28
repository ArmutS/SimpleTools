use std::fs;

// Stub for debugging
#[tauri::command(rename_all = "snake_case")]
pub fn convert_office(
    files: Vec<String>,
    targets: Vec<String>,
    output_dir: String,
) -> Result<String, String> {
    // Placeholder logic
    println!("Converting files: {:?}", files);
    println!("Targets: {:?}", targets);
    println!("Output Dir: {}", output_dir);

    // Simulate delay
    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(format!(
        "Successfully converted {} files to {:?}",
        files.len(),
        targets
    ))
}

#[tauri::command(rename_all = "snake_case")]
pub fn read_docx_binary(path: String) -> Result<Vec<u8>, String> {
    match fs::read(path) {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn save_binary_file(path: String, data: Vec<u8>) -> Result<(), String> {
    match fs::write(path, data) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}