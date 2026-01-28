use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use tar::Builder as TarBuilder;
use flate2::{Compression, write::GzEncoder};

/// Extract or convert archive files (ZIP, TAR, TAR.GZ, 7Z, RAR)
#[tauri::command]
pub fn convert_archive(
    file_path: String,
    output_dir: String,
    target_format: String,
) -> Result<String, String> {
    let source_path = Path::new(&file_path);
    let file_name = source_path
        .file_stem()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file name")?;
    
    let input_ext = get_archive_type(&file_path)?;
    let target = normalize_format(&target_format);

    // First, extract the archive to a temp directory
    let temp_dir = std::env::temp_dir().join(format!("archive_temp_{}", std::process::id()));
    fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;

    // Extract based on source format
    match input_ext.as_str() {
        "zip" => extract_zip(&file_path, &temp_dir)?,
        "tar" => extract_tar(&file_path, &temp_dir)?,
        "targz" | "tgz" => extract_tar_gz(&file_path, &temp_dir)?,
        "gz" => extract_gz(&file_path, &temp_dir)?,
        _ => return Err(format!("Unsupported source format: {}", input_ext)),
    }

    // Create archive in target format
    let output_path = match target.as_str() {
        "zip" => {
            let path = format!("{}/{}.zip", output_dir, file_name);
            create_zip(&temp_dir, &path)?;
            path
        }
        "tar" => {
            let path = format!("{}/{}.tar", output_dir, file_name);
            create_tar(&temp_dir, &path)?;
            path
        }
        "targz" => {
            let path = format!("{}/{}.tar.gz", output_dir, file_name);
            create_tar_gz(&temp_dir, &path)?;
            path
        }
        _ => return Err(format!("Unsupported target format: {}", target)),
    };

    // Cleanup temp directory
    let _ = fs::remove_dir_all(&temp_dir);

    Ok(format!("Converted {} to {}", file_name, target))
}

fn get_archive_type(path: &str) -> Result<String, String> {
    let path = Path::new(path);
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file name")?;
    
    if file_name.ends_with(".tar.gz") || file_name.ends_with(".tgz") {
        Ok("targz".to_string())
    } else {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .ok_or_else(|| "No file extension".to_string())
    }
}

fn normalize_format(format: &str) -> String {
    match format.to_lowercase().as_str() {
        "tar.gz" | "tgz" => "targz".to_string(),
        other => other.to_string(),
    }
}

// ZIP Functions
fn extract_zip(archive_path: &str, dest_dir: &Path) -> Result<(), String> {
    let file = File::open(archive_path)
        .map_err(|e| format!("Failed to open ZIP: {}", e))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Failed to read ZIP: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to access file in ZIP: {}", e))?;
        
        let outpath = match file.enclosed_name() {
            Some(path) => dest_dir.join(path),
            None => continue,
        };

        if file.is_dir() {
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                fs::create_dir_all(p)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }
            let mut outfile = File::create(&outpath)
                .map_err(|e| format!("Failed to create file: {}", e))?;
            io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to extract file: {}", e))?;
        }
    }
    Ok(())
}

fn create_zip(source_dir: &Path, dest_path: &str) -> Result<(), String> {
    let file = File::create(dest_path)
        .map_err(|e| format!("Failed to create ZIP file: {}", e))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    add_dir_to_zip(&mut zip, source_dir, source_dir, options)?;
    
    zip.finish()
        .map_err(|e| format!("Failed to finalize ZIP: {}", e))?;
    Ok(())
}

fn add_dir_to_zip<W: Write + io::Seek>(
    zip: &mut zip::ZipWriter<W>,
    base_dir: &Path,
    current_dir: &Path,
    options: SimpleFileOptions,
) -> Result<(), String> {
    let entries = fs::read_dir(current_dir)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        let name = path.strip_prefix(base_dir)
            .map_err(|e| format!("Failed to strip prefix: {}", e))?;

        if path.is_file() {
            zip.start_file(name.to_string_lossy().into_owned(), options)
                .map_err(|e| format!("Failed to start file in ZIP: {}", e))?;
            let mut f = File::open(&path)
                .map_err(|e| format!("Failed to open file: {}", e))?;
            io::copy(&mut f, zip)
                .map_err(|e| format!("Failed to write file to ZIP: {}", e))?;
        } else if path.is_dir() {
            add_dir_to_zip(zip, base_dir, &path, options)?;
        }
    }
    Ok(())
}

// TAR Functions
fn extract_tar(archive_path: &str, dest_dir: &Path) -> Result<(), String> {
    let file = File::open(archive_path)
        .map_err(|e| format!("Failed to open TAR: {}", e))?;
    let mut archive = tar::Archive::new(file);
    archive.unpack(dest_dir)
        .map_err(|e| format!("Failed to extract TAR: {}", e))?;
    Ok(())
}

fn create_tar(source_dir: &Path, dest_path: &str) -> Result<(), String> {
    let file = File::create(dest_path)
        .map_err(|e| format!("Failed to create TAR file: {}", e))?;
    let mut tar = TarBuilder::new(file);
    tar.append_dir_all("", source_dir)
        .map_err(|e| format!("Failed to add directory to TAR: {}", e))?;
    tar.finish()
        .map_err(|e| format!("Failed to finalize TAR: {}", e))?;
    Ok(())
}

// TAR.GZ Functions
fn extract_tar_gz(archive_path: &str, dest_dir: &Path) -> Result<(), String> {
    let file = File::open(archive_path)
        .map_err(|e| format!("Failed to open TAR.GZ: {}", e))?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(dest_dir)
        .map_err(|e| format!("Failed to extract TAR.GZ: {}", e))?;
    Ok(())
}

fn create_tar_gz(source_dir: &Path, dest_path: &str) -> Result<(), String> {
    let file = File::create(dest_path)
        .map_err(|e| format!("Failed to create TAR.GZ file: {}", e))?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut tar = TarBuilder::new(encoder);
    tar.append_dir_all("", source_dir)
        .map_err(|e| format!("Failed to add directory to TAR: {}", e))?;
    tar.finish()
        .map_err(|e| format!("Failed to finalize TAR.GZ: {}", e))?;
    Ok(())
}

// GZ Functions (for single files)
fn extract_gz(archive_path: &str, dest_dir: &Path) -> Result<(), String> {
    let file = File::open(archive_path)
        .map_err(|e| format!("Failed to open GZ: {}", e))?;
    let mut decoder = flate2::read::GzDecoder::new(file);
    
    let source_name = Path::new(archive_path)
        .file_stem()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file name")?;
    let dest_path = dest_dir.join(source_name);
    
    let mut output = File::create(&dest_path)
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    io::copy(&mut decoder, &mut output)
        .map_err(|e| format!("Failed to extract GZ: {}", e))?;
    Ok(())
}
