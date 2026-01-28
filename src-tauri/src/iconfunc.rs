use image::{ImageFormat, ImageReader, DynamicImage, imageops::FilterType};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Convert icon files (ICO, ICNS, PNG with multiple sizes)
#[tauri::command]
pub fn convert_icon(
    file_path: String,
    output_dir: String,
    target_format: String,
) -> Result<String, String> {
    let path = Path::new(&file_path);
    let file_name = path
        .file_stem()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file name")?;

    // Load image
    let img = ImageReader::open(&file_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    let target = target_format.to_lowercase();
    
    match target.as_str() {
        "ico" => {
            // Create Windows ICO with multiple sizes
            let output_path = format!("{}/{}.ico", output_dir, file_name);
            create_ico(&img, &output_path)?;
            Ok(format!("Created {}.ico", file_name))
        }
        "icns" => {
            // ICNS is complex, for now just save as PNG
            // Full ICNS support would need external crate
            let output_path = format!("{}/{}.png", output_dir, file_name);
            img.save_with_format(&output_path, ImageFormat::Png)
                .map_err(|e| format!("Failed to save PNG: {}", e))?;
            Ok(format!("Saved {} as PNG (ICNS not fully supported)", file_name))
        }
        "png" => {
            let output_path = format!("{}/{}.png", output_dir, file_name);
            img.save_with_format(&output_path, ImageFormat::Png)
                .map_err(|e| format!("Failed to save PNG: {}", e))?;
            Ok(format!("Converted {} to PNG", file_name))
        }
        _ => Err(format!("Unsupported format: {}", target)),
    }
}

fn create_ico(img: &DynamicImage, output_path: &str) -> Result<(), String> {
    // ICO format supports multiple sizes
    // We'll create common icon sizes: 16x16, 32x32, 48x48, 256x256
    let sizes = vec![16, 32, 48, 256];
    
    let file = File::create(output_path)
        .map_err(|e| format!("Failed to create ICO file: {}", e))?;
    let mut writer = BufWriter::new(file);

    // ICO header
    writer.write_all(&[0, 0]).map_err(|e| format!("Write error: {}", e))?; // Reserved
    writer.write_all(&[1, 0]).map_err(|e| format!("Write error: {}", e))?; // Type (1 = ICO)
    writer.write_all(&(sizes.len() as u16).to_le_bytes()).map_err(|e| format!("Write error: {}", e))?; // Number of images

    let mut image_data = Vec::new();
    let mut offset = 6 + (sizes.len() * 16); // Header + directory entries

    // Write directory entries and collect image data
    for size in &sizes {
        let resized = img.resize_exact(*size, *size, FilterType::Lanczos3);
        let mut png_data = Vec::new();
        resized.write_to(&mut std::io::Cursor::new(&mut png_data), ImageFormat::Png)
            .map_err(|e| format!("Failed to encode PNG: {}", e))?;

        // Directory entry
        writer.write_all(&[*size as u8]).map_err(|e| format!("Write error: {}", e))?; // Width
        writer.write_all(&[*size as u8]).map_err(|e| format!("Write error: {}", e))?; // Height
        writer.write_all(&[0]).map_err(|e| format!("Write error: {}", e))?; // Color palette
        writer.write_all(&[0]).map_err(|e| format!("Write error: {}", e))?; // Reserved
        writer.write_all(&[1, 0]).map_err(|e| format!("Write error: {}", e))?; // Color planes
        writer.write_all(&[32, 0]).map_err(|e| format!("Write error: {}", e))?; // Bits per pixel
        writer.write_all(&(png_data.len() as u32).to_le_bytes()).map_err(|e| format!("Write error: {}", e))?; // Size
        writer.write_all(&(offset as u32).to_le_bytes()).map_err(|e| format!("Write error: {}", e))?; // Offset

        offset += png_data.len();
        image_data.push(png_data);
    }

    // Write image data
    for data in image_data {
        writer.write_all(&data).map_err(|e| format!("Write error: {}", e))?;
    }

    Ok(())
}
