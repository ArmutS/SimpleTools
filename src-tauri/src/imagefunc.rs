use image::{ImageFormat, ImageReader};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Convert image files between formats (JPG, PNG, WebP, GIF, BMP, TIFF)
#[tauri::command]
pub fn convert_image(
    file_path: String,
    output_dir: String,
    target_format: String,
    quality: Option<u8>,
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

    // Determine output format
    let target = target_format.to_lowercase();
    let (format, extension) = match target.as_str() {
        "jpg" | "jpeg" => (ImageFormat::Jpeg, "jpg"),
        "png" => (ImageFormat::Png, "png"),
        "webp" => (ImageFormat::WebP, "webp"),
        "gif" => (ImageFormat::Gif, "gif"),
        "bmp" => (ImageFormat::Bmp, "bmp"),
        "tiff" | "tif" => (ImageFormat::Tiff, "tiff"),
        _ => return Err(format!("Unsupported format: {}", target)),
    };

    // Create output path
    let output_path = format!("{}/{}.{}", output_dir, file_name, extension);
    let output_file = File::create(&output_path)
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    let writer = BufWriter::new(output_file);

    // Save with format-specific settings
    match format {
        ImageFormat::Jpeg => {
            let quality = quality.unwrap_or(90);
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(writer, quality);
            encoder
                .encode_image(&img)
                .map_err(|e| format!("Failed to encode JPEG: {}", e))?;
        }
        ImageFormat::Png => {
            img.write_to(&mut std::io::BufWriter::new(writer), ImageFormat::Png)
                .map_err(|e| format!("Failed to encode PNG: {}", e))?;
        }
        ImageFormat::WebP => {
            // WebP is built-in but simpler API
            img.write_to(&mut std::io::BufWriter::new(writer), ImageFormat::WebP)
                .map_err(|e| format!("Failed to encode WebP: {}", e))?;
        }
        _ => {
            // Generic encoding for other formats
            img.write_to(&mut std::io::BufWriter::new(writer), format)
                .map_err(|e| format!("Failed to encode image: {}", e))?;
        }
    }

    Ok(format!("Converted {} to {}", file_name, extension))
}

/// Resize and convert image
#[tauri::command]
pub fn resize_image(
    file_path: String,
    output_dir: String,
    target_format: String,
    width: Option<u32>,
    height: Option<u32>,
    maintain_aspect: bool,
) -> Result<String, String> {
    let path = Path::new(&file_path);
    let file_name = path
        .file_stem()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file name")?;

    // Load image
    let mut img = ImageReader::open(&file_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    // Resize if dimensions provided
    if let (Some(w), Some(h)) = (width, height) {
        img = if maintain_aspect {
            img.resize(w, h, image::imageops::FilterType::Lanczos3)
        } else {
            img.resize_exact(w, h, image::imageops::FilterType::Lanczos3)
        };
    } else if let Some(w) = width {
        let h = (img.height() as f32 * w as f32 / img.width() as f32) as u32;
        img = img.resize(w, h, image::imageops::FilterType::Lanczos3);
    } else if let Some(h) = height {
        let w = (img.width() as f32 * h as f32 / img.height() as f32) as u32;
        img = img.resize(w, h, image::imageops::FilterType::Lanczos3);
    }

    // Determine output format
    let target = target_format.to_lowercase();
    let (format, extension) = match target.as_str() {
        "jpg" | "jpeg" => (ImageFormat::Jpeg, "jpg"),
        "png" => (ImageFormat::Png, "png"),
        "webp" => (ImageFormat::WebP, "webp"),
        "gif" => (ImageFormat::Gif, "gif"),
        "bmp" => (ImageFormat::Bmp, "bmp"),
        "tiff" | "tif" => (ImageFormat::Tiff, "tiff"),
        _ => return Err(format!("Unsupported format: {}", target)),
    };

    // Save
    let output_path = format!("{}/{}_resized.{}", output_dir, file_name, extension);
    img.save_with_format(&output_path, format)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(format!("Resized {} to {}", file_name, extension))
}
