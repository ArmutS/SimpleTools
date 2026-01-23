// Stub for debugging
#[tauri::command(rename_all = "snake_case")]
pub fn convert_office(files: Vec<String>, targets: Vec<String>, output_dir: String) -> Result<String, String> {
    // Placeholder logic
    println!("Converting files: {:?}", files);
    println!("Targets: {:?}", targets);
    println!("Output Dir: {}", output_dir);

    // Simulate delay
    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(format!("Successfully converted {} files to {:?}", files.len(), targets))
}
