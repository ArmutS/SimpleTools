use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use tauri::command;

// ============================================================================
// 1. HASH GENERATOR (H)
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct HashResult {
    md5: String,
    sha256: String,
    sha512: String,
}

#[command]
pub fn hash_generate(input: String, is_file: bool) -> Result<HashResult, String> {
    use md5::Md5;
    use sha2::{Digest, Sha256, Sha512};

    let data: Vec<u8> = if is_file {
        fs::read(&input).map_err(|e| format!("Failed to read file: {}", e))?
    } else {
        input.as_bytes().to_vec()
    };

    let md5_hash = {
        let mut hasher = Md5::new();
        hasher.update(&data);
        format!("{:x}", hasher.finalize())
    };
    
    let sha256_hash = {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        format!("{:x}", hasher.finalize())
    };
    
    let sha512_hash = {
        let mut hasher = Sha512::new();
        hasher.update(&data);
        format!("{:x}", hasher.finalize())
    };

    Ok(HashResult {
        md5: md5_hash,
        sha256: sha256_hash,
        sha512: sha512_hash,
    })
}

// ============================================================================
// 2. FILE RENAMER (R)
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RenamePreview {
    old_name: String,
    new_name: String,
    full_old_path: String,
    full_new_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameRequest {
    files: Vec<String>,
    find_pattern: String,
    replace_with: String,
    use_regex: bool,
}

#[command]
pub fn file_rename_batch(
    request: RenameRequest,
    preview_only: bool,
) -> Result<Vec<RenamePreview>, String> {
    let mut results = Vec::new();

    for file_path in &request.files {
        let path = Path::new(file_path);
        if !path.exists() {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("Invalid filename")?;

        let new_name = if request.use_regex {
            let re = regex::Regex::new(&request.find_pattern)
                .map_err(|e| format!("Invalid regex: {}", e))?;
            re.replace_all(file_name, &request.replace_with).to_string()
        } else {
            file_name.replace(&request.find_pattern, &request.replace_with)
        };

        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        let new_path = parent.join(&new_name);

        results.push(RenamePreview {
            old_name: file_name.to_string(),
            new_name: new_name.clone(),
            full_old_path: file_path.clone(),
            full_new_path: new_path.to_string_lossy().to_string(),
        });

        if !preview_only {
            fs::rename(path, &new_path)
                .map_err(|e| format!("Failed to rename {}: {}", file_name, e))?;
        }
    }

    Ok(results)
}

// ============================================================================
// 3. DUPLICATE FINDER (D)
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DuplicateGroup {
    hash: String,
    files: Vec<FileInfo>,
    total_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    path: String,
    size: u64,
}

#[command]
pub fn find_duplicates(directory: String) -> Result<Vec<DuplicateGroup>, String> {
    use sha2::{Digest, Sha256};
    let mut hash_map: HashMap<String, Vec<FileInfo>> = HashMap::new();

    fn scan_directory(dir: &Path, hash_map: &mut HashMap<String, Vec<FileInfo>>) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() {
                    let metadata = fs::metadata(&path)?;
                    let size = metadata.len();
                    
                    // Calculate SHA256 hash
                    let mut file = File::open(&path)?;
                    let mut hasher = Sha256::new();
                    let mut buffer = [0; 8192];
                    
                    loop {
                        let n = file.read(&mut buffer)?;
                        if n == 0 {
                            break;
                        }
                        hasher.update(&buffer[..n]);
                    }
                    
                    let hash = format!("{:x}", hasher.finalize());
                    
                    hash_map.entry(hash).or_insert_with(Vec::new).push(FileInfo {
                        path: path.to_string_lossy().to_string(),
                        size,
                    });
                } else if path.is_dir() {
                    scan_directory(&path, hash_map)?;
                }
            }
        }
        Ok(())
    }

    let dir_path = Path::new(&directory);
    scan_directory(dir_path, &mut hash_map)
        .map_err(|e| format!("Failed to scan directory: {}", e))?;

    // Filter only duplicates (more than 1 file with same hash)
    let duplicates: Vec<DuplicateGroup> = hash_map
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .map(|(hash, files)| {
            let total_size: u64 = files.iter().map(|f| f.size).sum();
            DuplicateGroup {
                hash,
                files,
                total_size,
            }
        })
        .collect();

    Ok(duplicates)
}

// ============================================================================
// 4. DISK USAGE ANALYZER (U)
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskUsageNode {
    path: String,
    name: String,
    size: u64,
    is_dir: bool,
    children: Vec<DiskUsageNode>,
}

#[command]
pub fn analyze_disk_usage(directory: String, max_depth: Option<usize>) -> Result<DiskUsageNode, String> {
    fn calculate_size(path: &Path, current_depth: usize, max_depth: Option<usize>) -> io::Result<DiskUsageNode> {
        let metadata = fs::metadata(path)?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        if metadata.is_file() {
            Ok(DiskUsageNode {
                path: path.to_string_lossy().to_string(),
                name,
                size: metadata.len(),
                is_dir: false,
                children: vec![],
            })
        } else {
            let mut children = Vec::new();
            let mut total_size = 0u64;

            if max_depth.is_none() || current_depth < max_depth.unwrap() {
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let child_node = calculate_size(&entry.path(), current_depth + 1, max_depth)?;
                    total_size += child_node.size;
                    children.push(child_node);
                }
            }

            // Sort children by size (largest first)
            children.sort_by(|a, b| b.size.cmp(&a.size));

            Ok(DiskUsageNode {
                path: path.to_string_lossy().to_string(),
                name,
                size: total_size,
                is_dir: true,
                children,
            })
        }
    }

    let dir_path = Path::new(&directory);
    calculate_size(dir_path, 0, max_depth)
        .map_err(|e| format!("Failed to analyze disk usage: {}", e))
}

// ============================================================================
// 5. FILE SPLITTER (S)
// ============================================================================

#[command]
pub fn file_split(
    file_path: String,
    chunk_size_mb: u64,
    output_dir: String,
) -> Result<Vec<String>, String> {
    let path = Path::new(&file_path);
    let chunk_size = chunk_size_mb * 1024 * 1024; // Convert MB to bytes
    
    let mut file = File::open(path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid filename")?;
    
    let output_path = Path::new(&output_dir);
    fs::create_dir_all(output_path)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;
    
    let mut chunk_paths = Vec::new();
    let mut chunk_number = 1;
    let mut buffer = vec![0u8; chunk_size as usize];
    
    loop {
        let bytes_read = file.read(&mut buffer)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        if bytes_read == 0 {
            break;
        }
        
        let chunk_file_name = format!("{}.part{:03}", file_name, chunk_number);
        let chunk_path = output_path.join(&chunk_file_name);
        
        let mut chunk_file = File::create(&chunk_path)
            .map_err(|e| format!("Failed to create chunk file: {}", e))?;
        
        chunk_file.write_all(&buffer[..bytes_read])
            .map_err(|e| format!("Failed to write chunk: {}", e))?;
        
        chunk_paths.push(chunk_path.to_string_lossy().to_string());
        chunk_number += 1;
    }
    
    Ok(chunk_paths)
}

#[command]
pub fn file_merge(chunk_files: Vec<String>, output_file: String) -> Result<String, String> {
    let mut output = File::create(&output_file)
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    
    for chunk_path in &chunk_files {
        let mut chunk = File::open(chunk_path)
            .map_err(|e| format!("Failed to open chunk {}: {}", chunk_path, e))?;
        
        io::copy(&mut chunk, &mut output)
            .map_err(|e| format!("Failed to merge chunk: {}", e))?;
    }
    
    Ok(output_file)
}

// ============================================================================
// 6. CHECKSUM VERIFIER (C)
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ChecksumResult {
    calculated: String,
    expected: String,
    matches: bool,
    algorithm: String,
}

#[command]
pub fn checksum_verify(
    file_path: String,
    expected_checksum: String,
    algorithm: String, // "md5", "sha256", "sha512"
) -> Result<ChecksumResult, String> {
    use md5::Md5;
    use sha2::{Digest, Sha256, Sha512};

    let data = fs::read(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let calculated = match algorithm.to_lowercase().as_str() {
        "md5" => {
            let mut hasher = Md5::new();
            hasher.update(&data);
            format!("{:x}", hasher.finalize())
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(&data);
            format!("{:x}", hasher.finalize())
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(&data);
            format!("{:x}", hasher.finalize())
        }
        _ => return Err(format!("Unsupported algorithm: {}", algorithm)),
    };

    let matches = calculated.eq_ignore_ascii_case(&expected_checksum);

    Ok(ChecksumResult {
        calculated,
        expected: expected_checksum,
        matches,
        algorithm,
    })
}

// ============================================================================
// 7. FILE PERMISSIONS (P) - Platform Specific
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionsInfo {
    octal: String,
    symbolic: String,
    readable: bool,
    writable: bool,
    executable: bool,
}

#[cfg(unix)]
#[command]
pub fn get_file_permissions(file_path: String) -> Result<PermissionsInfo, String> {
    use std::os::unix::fs::PermissionsExt;
    
    let metadata = fs::metadata(&file_path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;
    
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    
    // Extract permission bits (last 9 bits)
    let perms = mode & 0o777;
    let octal = format!("{:o}", perms);
    
    // Convert to symbolic notation
    let symbolic = format!(
        "{}{}{}{}{}{}{}{}{}",
        if perms & 0o400 != 0 { 'r' } else { '-' },
        if perms & 0o200 != 0 { 'w' } else { '-' },
        if perms & 0o100 != 0 { 'x' } else { '-' },
        if perms & 0o040 != 0 { 'r' } else { '-' },
        if perms & 0o020 != 0 { 'w' } else { '-' },
        if perms & 0o010 != 0 { 'x' } else { '-' },
        if perms & 0o004 != 0 { 'r' } else { '-' },
        if perms & 0o002 != 0 { 'w' } else { '-' },
        if perms & 0o001 != 0 { 'x' } else { '-' },
    );
    
    Ok(PermissionsInfo {
        octal,
        symbolic,
        readable: permissions.readonly() == false,
        writable: permissions.readonly() == false,
        executable: perms & 0o111 != 0,
    })
}

#[cfg(windows)]
#[command]
pub fn get_file_permissions(file_path: String) -> Result<PermissionsInfo, String> {
    let metadata = fs::metadata(&file_path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;
    
    let permissions = metadata.permissions();
    let readonly = permissions.readonly();
    
    Ok(PermissionsInfo {
        octal: "N/A".to_string(),
        symbolic: if readonly { "r--" } else { "rw-" }.to_string(),
        readable: true,
        writable: !readonly,
        executable: false, // Windows doesn't have simple executable bit
    })
}

#[cfg(target_os = "macos")]
#[command]
pub fn get_file_permissions(file_path: String) -> Result<PermissionsInfo, String> {
    // macOS is Unix-like, so we use the Unix implementation
    use std::os::unix::fs::PermissionsExt;
    
    let metadata = fs::metadata(&file_path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;
    
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    let perms = mode & 0o777;
    let octal = format!("{:o}", perms);
    
    let symbolic = format!(
        "{}{}{}{}{}{}{}{}{}",
        if perms & 0o400 != 0 { 'r' } else { '-' },
        if perms & 0o200 != 0 { 'w' } else { '-' },
        if perms & 0o100 != 0 { 'x' } else { '-' },
        if perms & 0o040 != 0 { 'r' } else { '-' },
        if perms & 0o020 != 0 { 'w' } else { '-' },
        if perms & 0o010 != 0 { 'x' } else { '-' },
        if perms & 0o004 != 0 { 'r' } else { '-' },
        if perms & 0o002 != 0 { 'w' } else { '-' },
        if perms & 0o001 != 0 { 'x' } else { '-' },
    );
    
    Ok(PermissionsInfo {
        octal,
        symbolic,
        readable: permissions.readonly() == false,
        writable: permissions.readonly() == false,
        executable: perms & 0o111 != 0,
    })
}

// ============================================================================
// 8. DIRECTORY TREE (T)
// ============================================================================

#[command]
pub fn generate_directory_tree(
    directory: String,
    max_depth: Option<usize>,
    filter_extension: Option<String>,
) -> Result<String, String> {
    fn build_tree(
        path: &Path,
        prefix: &str,
        is_last: bool,
        current_depth: usize,
        max_depth: Option<usize>,
        filter_ext: &Option<String>,
        output: &mut String,
    ) -> io::Result<()> {
        if max_depth.is_some() && current_depth >= max_depth.unwrap() {
            return Ok(());
        }

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Apply filter
        if let Some(ext) = filter_ext {
            if path.is_file() {
                if let Some(file_ext) = path.extension().and_then(|e| e.to_str()) {
                    if file_ext != ext {
                        return Ok(());
                    }
                } else {
                    return Ok(());
                }
            }
        }

        let branch = if is_last { "└── " } else { "├── " };
        output.push_str(&format!("{}{}{}\n", prefix, branch, name));

        if path.is_dir() {
            let mut entries: Vec<_> = fs::read_dir(path)?
                .filter_map(|e| e.ok())
                .collect();
            entries.sort_by_key(|e| e.path());

            let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

            for (i, entry) in entries.iter().enumerate() {
                let is_last_child = i == entries.len() - 1;
                build_tree(
                    &entry.path(),
                    &child_prefix,
                    is_last_child,
                    current_depth + 1,
                    max_depth,
                    filter_ext,
                    output,
                )?;
            }
        }

        Ok(())
    }

    let dir_path = Path::new(&directory);
    let mut tree_output = String::new();
    
    // Root
    tree_output.push_str(&format!("{}\n", directory));
    
    let entries: Vec<_> = fs::read_dir(dir_path)
        .map_err(|e| format!("Failed to read directory: {}", e))?
        .filter_map(|e| e.ok())
        .collect();

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        build_tree(
            &entry.path(),
            "",
            is_last,
            0,
            max_depth,
            &filter_extension,
            &mut tree_output,
        )
        .map_err(|e| format!("Failed to build tree: {}", e))?;
    }

    Ok(tree_output)
}

// ============================================================================
// 9. FILE WATCHER (W) - Requires notify crate
// ============================================================================

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchEvent {
    event_type: String,
    paths: Vec<String>,
    timestamp: String,
}

#[command]
pub fn watch_directory(directory: String, duration_secs: u64) -> Result<Vec<WatchEvent>, String> {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default(),
    )
    .map_err(|e| format!("Failed to create watcher: {}", e))?;

    watcher
        .watch(Path::new(&directory), RecursiveMode::Recursive)
        .map_err(|e| format!("Failed to watch directory: {}", e))?;

    let mut events = Vec::new();
    let start = std::time::Instant::now();

    while start.elapsed().as_secs() < duration_secs {
        if let Ok(event) = rx.recv_timeout(Duration::from_millis(100)) {
            let event_type = format!("{:?}", event.kind);
            let paths: Vec<String> = event
                .paths
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();

            events.push(WatchEvent {
                event_type,
                paths,
                timestamp: chrono::Local::now().to_rfc3339(),
            });
        }
    }

    Ok(events)
}

// ============================================================================
// 10. TEMP FILE CLEANER (L)
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct TempFileInfo {
    path: String,
    size: u64,
    age_days: u64,
}

#[command]
pub fn clean_temp_files(
    scan_only: bool,
    min_age_days: Option<u64>,
) -> Result<Vec<TempFileInfo>, String> {
    let temp_dirs = if cfg!(windows) {
        vec![
            std::env::var("TEMP").unwrap_or_default(),
            std::env::var("TMP").unwrap_or_default(),
        ]
    } else {
        vec!["/tmp".to_string(), "/var/tmp".to_string()]
    };

    let mut temp_files = Vec::new();
    let now = std::time::SystemTime::now();

    for temp_dir in temp_dirs {
        let path = Path::new(&temp_dir);
        if !path.exists() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Ok(modified) = metadata.modified() {
                            if let Ok(duration) = now.duration_since(modified) {
                                let age_days = duration.as_secs() / 86400;
                                
                                if let Some(min_age) = min_age_days {
                                    if age_days < min_age {
                                        continue;
                                    }
                                }

                                let file_path = entry.path().to_string_lossy().to_string();
                                temp_files.push(TempFileInfo {
                                    path: file_path.clone(),
                                    size: metadata.len(),
                                    age_days,
                                });

                                if !scan_only {
                                    let _ = fs::remove_file(&file_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(temp_files)
}

// ============================================================================
// 11. FILE METADATA VIEWER (M) - EXIF & ID3
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    file_name: String,
    file_size: u64,
    file_type: String,
    created: Option<String>,
    modified: Option<String>,
    exif_data: Option<HashMap<String, String>>,
    id3_data: Option<HashMap<String, String>>,
}

#[command]
pub fn get_file_metadata(file_path: String) -> Result<FileMetadata, String> {
    let path = Path::new(&file_path);
    let metadata = fs::metadata(path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    let file_type = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("unknown")
        .to_string();

    let created = metadata
        .created()
        .ok()
        .and_then(|t| chrono::DateTime::<chrono::Local>::from(t).to_rfc3339().into());

    let modified = metadata
        .modified()
        .ok()
        .and_then(|t| chrono::DateTime::<chrono::Local>::from(t).to_rfc3339().into());

    // Try to read EXIF data for images
    let exif_data = if matches!(
        file_type.to_lowercase().as_str(),
        "jpg" | "jpeg" | "tiff" | "tif"
    ) {
        read_exif_data(&file_path).ok()
    } else {
        None
    };

    // Try to read ID3 data for audio
    let id3_data = if file_type.to_lowercase() == "mp3" {
        read_id3_data(&file_path).ok()
    } else {
        None
    };

    Ok(FileMetadata {
        file_name,
        file_size: metadata.len(),
        file_type,
        created,
        modified,
        exif_data,
        id3_data,
    })
}

fn read_exif_data(file_path: &str) -> Result<HashMap<String, String>, String> {
    use exif::Reader;
    
    let file = File::open(file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    let mut bufreader = BufReader::new(file);
    
    let exifreader = Reader::new()
        .read_from_container(&mut bufreader)
        .map_err(|e| format!("Failed to read EXIF: {}", e))?;

    let mut data = HashMap::new();
    
    for field in exifreader.fields() {
        let tag_name = format!("{:?}", field.tag);
        let value = field.display_value().with_unit(&exifreader).to_string();
        data.insert(tag_name, value);
    }

    Ok(data)
}

fn read_id3_data(file_path: &str) -> Result<HashMap<String, String>, String> {
    let tag = id3::Tag::read_from_path(file_path)
        .map_err(|e| format!("Failed to read ID3: {}", e))?;

    let mut data = HashMap::new();
    
    // Iterate through frames
    for frame in tag.frames() {
        match frame.id() {
            "TIT2" => {
                if let Some(text) = frame.content().text() {
                    data.insert("Title".to_string(), text.to_string());
                }
            }
            "TPE1" => {
                if let Some(text) = frame.content().text() {
                    data.insert("Artist".to_string(), text.to_string());
                }
            }
            "TALB" => {
                if let Some(text) = frame.content().text() {
                    data.insert("Album".to_string(), text.to_string());
                }
            }
            "TDRC" => {
                if let Some(text) = frame.content().text() {
                    data.insert("Year".to_string(), text.to_string());
                }
            }
            "TCON" => {
                if let Some(text) = frame.content().text() {
                    data.insert("Genre".to_string(), text.to_string());
                }
            }
            _ => {}
        }
    }

    Ok(data)
}

// ============================================================================
// 12. SYMBOLIC LINK MANAGER (Y) - Platform Specific
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SymlinkInfo {
    link_path: String,
    target_path: String,
    exists: bool,
}

#[cfg(unix)]
#[command]
pub fn create_symlink(source: String, link_path: String) -> Result<String, String> {
    use std::os::unix::fs as unix_fs;
    
    unix_fs::symlink(&source, &link_path)
        .map_err(|e| format!("Failed to create symlink: {}", e))?;
    
    Ok(format!("Symlink created: {} -> {}", link_path, source))
}

#[cfg(windows)]
#[command]
pub fn create_symlink(source: String, link_path: String) -> Result<String, String> {
    use std::os::windows::fs as windows_fs;
    
    let src_path = Path::new(&source);
    
    if src_path.is_dir() {
        windows_fs::symlink_dir(&source, &link_path)
            .map_err(|e| format!("Failed to create directory symlink: {}", e))?;
    } else {
        windows_fs::symlink_file(&source, &link_path)
            .map_err(|e| format!("Failed to create file symlink: {}", e))?;
    }
    
    Ok(format!("Symlink created: {} -> {}", link_path, source))
}

#[cfg(target_os = "macos")]
#[command]
pub fn create_symlink(source: String, link_path: String) -> Result<String, String> {
    use std::os::unix::fs as unix_fs;
    
    unix_fs::symlink(&source, &link_path)
        .map_err(|e| format!("Failed to create symlink: {}", e))?;
    
    Ok(format!("Symlink created: {} -> {}", link_path, source))
}

#[command]
pub fn list_symlinks(directory: String) -> Result<Vec<SymlinkInfo>, String> {
    let dir_path = Path::new(&directory);
    let mut symlinks = Vec::new();

    fn scan_for_symlinks(path: &Path, results: &mut Vec<SymlinkInfo>) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Ok(metadata) = fs::symlink_metadata(&path) {
                if metadata.file_type().is_symlink() {
                    let link_path = path.to_string_lossy().to_string();
                    let target = fs::read_link(&path)
                        .unwrap_or_else(|_| PathBuf::from("Unknown"))
                        .to_string_lossy()
                        .to_string();
                    
                    let exists = Path::new(&target).exists();
                    
                    results.push(SymlinkInfo {
                        link_path,
                        target_path: target,
                        exists,
                    });
                } else if metadata.is_dir() {
                    scan_for_symlinks(&path, results)?;
                }
            }
        }
        Ok(())
    }

    scan_for_symlinks(dir_path, &mut symlinks)
        .map_err(|e| format!("Failed to scan for symlinks: {}", e))?;

    Ok(symlinks)
}
