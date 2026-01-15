use lopdf::Document;
use serde::{Deserialize, Serialize};
use std::fs;

// ============================================================================
// PDF INFO (Metadata Reader)
// ============================================================================
#[derive(Serialize)]
pub struct PdfInfo {
    pub page_count: u32,
    pub file_size: u64,
    pub file_size_formatted: String,
    pub is_encrypted: bool,
    pub error: Option<String>,
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_pdf_info(file_path: String) -> PdfInfo {
    // Get file size
    let file_size = match fs::metadata(&file_path) {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            return PdfInfo {
                page_count: 0,
                file_size: 0,
                file_size_formatted: "Unknown".to_string(),
                is_encrypted: false,
                error: Some(format!("Cannot read file: {}", e)),
            }
        }
    };

    // Format file size
    let file_size_formatted = format_file_size(file_size);

    // Try to load PDF
    match Document::load(&file_path) {
        Ok(doc) => {
            let page_count = doc.get_pages().len() as u32;
            let is_encrypted = doc.is_encrypted();

            PdfInfo {
                page_count,
                file_size,
                file_size_formatted,
                is_encrypted,
                error: None,
            }
        }
        Err(e) => PdfInfo {
            page_count: 0,
            file_size,
            file_size_formatted,
            is_encrypted: false,
            error: Some(format!("Cannot read PDF: {}", e)),
        },
    }
}

fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

// ============================================================================
// OPEN FOLDER / FILE
// ============================================================================
#[tauri::command(rename_all = "snake_case")]
pub fn open_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn open_file(path: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", "", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

// ============================================================================
// PDF MERGER (Birleştirici)
// ============================================================================
#[derive(Deserialize)]
pub struct MergeRequest {
    pub files: Vec<String>,
    pub output_path: String,
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_merge(request: MergeRequest) -> Result<String, String> {
    if request.files.is_empty() {
        return Err("No files provided for merging".to_string());
    }

    if request.files.len() < 2 {
        return Err("At least 2 PDF files are required for merging".to_string());
    }

    // Load the first document
    let mut merged_doc = Document::load(&request.files[0])
        .map_err(|e| format!("Failed to load {}: {}", request.files[0], e))?;

    // Merge remaining documents
    for (_index, file_path) in request.files.iter().skip(1).enumerate() {
        let doc = Document::load(file_path)
            .map_err(|e| format!("Failed to load {}: {}", file_path, e))?;

        // Get the maximum page ID from merged document
        let mut max_id = merged_doc
            .get_pages()
            .into_values()
            .map(|id| id.0)
            .max()
            .unwrap_or(0);

        // Merge pages from current document
        for (_page_num, page_id) in doc.get_pages() {
            max_id += 1;
            let new_page_id = (max_id, 0);

            // Clone the page and its resources
            if let Ok(page_dict) = doc.get_object(page_id) {
                merged_doc.objects.insert(new_page_id, page_dict.clone());

                // Add to pages
                if let Ok(pages) = merged_doc.get_object_mut((1, 0)) {
                    if let Ok(kids) = pages.as_dict_mut()
                        .and_then(|d| d.get_mut(b"Kids"))
                        .and_then(|k| k.as_array_mut())
                    {
                        kids.push(lopdf::Object::Reference(new_page_id));
                    }
                }
            }
        }
    }

    // Save the merged document
    merged_doc
        .save(&request.output_path)
        .map_err(|e| format!("Failed to save merged PDF: {}", e))?;

    Ok(format!(
        "Successfully merged {} PDF files into {}",
        request.files.len(),
        request.output_path
    ))
}

// ============================================================================
// PDF SPLITTER (Ayırıcı)
// ============================================================================
#[derive(Deserialize)]
pub struct SplitRequest {
    pub file_path: String,
    pub mode: String, // "range" or "individual"
    pub start_page: Option<u32>,
    pub end_page: Option<u32>,
    pub output_dir: String,
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_split(request: SplitRequest) -> Result<String, String> {
    let doc = Document::load(&request.file_path)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;

    let pages = doc.get_pages();
    let total_pages = pages.len() as u32;

    match request.mode.as_str() {
        "range" => {
            let start = request.start_page.unwrap_or(1).max(1);
            let end = request.end_page.unwrap_or(total_pages).min(total_pages);

            if start > end || start > total_pages {
                return Err(format!("Invalid page range: {}-{}", start, end));
            }

            // Create new document with selected pages
            let mut new_doc = Document::with_version("1.5");
            let pages_to_extract: Vec<_> = pages.iter()
                .filter(|(num, _)| **num >= start && **num <= end)
                .collect();

            for (_, page_id) in pages_to_extract {
                if let Ok(page) = doc.get_object(*page_id) {
                    new_doc.objects.insert(*page_id, page.clone());
                }
            }

            let output_path = format!("{}/pages_{}-{}.pdf", request.output_dir, start, end);
            new_doc.save(&output_path)
                .map_err(|e| format!("Failed to save: {}", e))?;

            Ok(format!("Extracted pages {}-{} to {}", start, end, output_path))
        }
        "individual" => {
            let mut saved_count = 0;
            for (page_num, page_id) in pages {
                let mut new_doc = Document::with_version("1.5");
                if let Ok(page) = doc.get_object(page_id) {
                    new_doc.objects.insert(page_id, page.clone());
                    let output_path = format!("{}/page_{}.pdf", request.output_dir, page_num);
                    new_doc.save(&output_path)
                        .map_err(|e| format!("Failed to save page {}: {}", page_num, e))?;
                    saved_count += 1;
                }
            }
            Ok(format!("Split {} pages into separate files in {}", saved_count, request.output_dir))
        }
        _ => Err("Invalid split mode. Use 'range' or 'individual'".to_string()),
    }
}

// ============================================================================
// IMAGES TO PDF (Resimden PDF'e)
// ============================================================================
#[derive(Deserialize)]
pub struct ImagesToPdfRequest {
    pub image_paths: Vec<String>,
    pub output_path: String,
    pub page_size: Option<String>, // "A4", "Letter", etc.
}

#[tauri::command(rename_all = "snake_case")]
pub fn images_to_pdf(request: ImagesToPdfRequest) -> Result<String, String> {
    // TODO: Implement image to PDF conversion
    // Use image crate to load images and pdf-rs to create PDF
    
    if request.image_paths.is_empty() {
        return Err("No images provided".to_string());
    }
    
    Ok(format!(
        "Images to PDF: {} images would be converted to {}",
        request.image_paths.len(),
        request.output_path
    ))
}

// ============================================================================
// PDF TO IMAGES (PDF'ten Resme)
// ============================================================================
#[derive(Deserialize)]
pub struct PdfToImagesRequest {
    pub file_path: String,
    pub output_dir: String,
    pub format: String, // "png" or "jpg"
    pub dpi: Option<u32>,
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_to_images(request: PdfToImagesRequest) -> Result<String, String> {
    // TODO: Implement PDF to images conversion
    // Use pdf rendering library to convert each page to image
    
    let dpi = request.dpi.unwrap_or(150);
    Ok(format!(
        "PDF to Images: {} would be converted to {} images at {} DPI",
        request.file_path, request.format, dpi
    ))
}

// ============================================================================
// COMPRESS PDF (Sıkıştırıcı)
// ============================================================================
#[derive(Deserialize)]
pub struct CompressRequest {
    pub file_path: String,
    pub output_path: String,
    pub quality: Option<String>, // "low", "medium", "high"
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_compress(request: CompressRequest) -> Result<String, String> {
    // TODO: Implement PDF compression
    // Remove metadata, compress images, optimize structure
    
    let quality = request.quality.unwrap_or_else(|| "medium".to_string());
    Ok(format!(
        "PDF Compress: {} would be compressed with {} quality to {}",
        request.file_path, quality, request.output_path
    ))
}

// ============================================================================
// ROTATE PAGES (Sayfa Döndür)
// ============================================================================
#[derive(Deserialize)]
pub struct RotateRequest {
    pub file_path: String,
    pub output_path: String,
    pub pages: Vec<u32>, // Page numbers to rotate (empty = all pages)
    pub rotation: i32,   // 90, 180, 270, -90
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_rotate(request: RotateRequest) -> Result<String, String> {
    let mut doc = Document::load(&request.file_path)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;

    let pages = doc.get_pages();
    let pages_to_rotate: Vec<u32> = if request.pages.is_empty() {
        pages.keys().copied().collect()
    } else {
        request.pages.clone()
    };

    // Validate rotation angle
    if ![90, 180, 270, -90, -180, -270].contains(&request.rotation) {
        return Err("Rotation must be 90, 180, 270, -90, -180, or -270 degrees".to_string());
    }

    // Normalize rotation to positive value
    let rotation = ((request.rotation % 360) + 360) % 360;

    for page_num in pages_to_rotate {
        if let Some(page_id) = pages.get(&page_num) {
            if let Ok(page_obj) = doc.get_object_mut(*page_id) {
                if let Ok(page_dict) = page_obj.as_dict_mut() {
                    // Get current rotation or default to 0
                    let current_rotation = page_dict
                        .get(b"Rotate")
                        .and_then(|r| r.as_i64())
                        .unwrap_or(0) as i32;

                    // Calculate new rotation
                    let new_rotation = (current_rotation + rotation) % 360;

                    // Set new rotation
                    page_dict.set("Rotate", lopdf::Object::Integer(new_rotation as i64));
                }
            }
        }
    }

    doc.save(&request.output_path)
        .map_err(|e| format!("Failed to save: {}", e))?;

    let page_desc = if request.pages.is_empty() {
        "all pages".to_string()
    } else {
        format!("{} pages", request.pages.len())
    };

    Ok(format!(
        "Rotated {} by {} degrees in {}",
        page_desc, request.rotation, request.output_path
    ))
}

// ============================================================================
// DELETE PAGES (Sayfa Sil)
// ============================================================================
#[derive(Deserialize)]
pub struct DeletePagesRequest {
    pub file_path: String,
    pub output_path: String,
    pub pages_to_delete: Vec<u32>,
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_delete_pages(request: DeletePagesRequest) -> Result<String, String> {
    if request.pages_to_delete.is_empty() {
        return Err("No pages specified for deletion".to_string());
    }

    let doc = Document::load(&request.file_path)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;

    let pages = doc.get_pages();
    let pages_to_keep: Vec<_> = pages.iter()
        .filter(|(num, _)| !request.pages_to_delete.contains(num))
        .map(|(_, id)| *id)
        .collect();

    // Create new document with only kept pages
    let mut new_doc = Document::with_version("1.5");
    for page_id in pages_to_keep {
        if let Ok(page) = doc.get_object(page_id) {
            new_doc.objects.insert(page_id, page.clone());
        }
    }

    new_doc.save(&request.output_path)
        .map_err(|e| format!("Failed to save: {}", e))?;

    Ok(format!(
        "Deleted {} pages from {} and saved to {}",
        request.pages_to_delete.len(),
        request.file_path,
        request.output_path
    ))
}

// ============================================================================
// EXTRACT TEXT (Metin Kazıyıcı)
// ============================================================================
#[derive(Deserialize)]
pub struct ExtractTextRequest {
    pub file_path: String,
    pub pages: Option<Vec<u32>>, // None = all pages
}

#[derive(Serialize)]
pub struct ExtractTextResult {
    pub text: String,
    pub page_count: u32,
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_extract_text(request: ExtractTextRequest) -> Result<ExtractTextResult, String> {
    let doc = Document::load(&request.file_path)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;

    let pages = doc.get_pages();
    let pages_to_extract: Vec<u32> = request.pages.clone().unwrap_or_else(|| {
        pages.keys().copied().collect()
    });

    let mut extracted_text = String::new();
    let mut page_count = 0;

    for page_num in pages_to_extract {
        if let Some(_page_id) = pages.get(&page_num) {
            if let Ok(content) = doc.extract_text(&[page_num]) {
                extracted_text.push_str(&format!("\n--- Page {} ---\n", page_num));
                extracted_text.push_str(&content);
                page_count += 1;
            }
        }
    }

    if extracted_text.is_empty() {
        extracted_text = "No text found in PDF".to_string();
    }

    Ok(ExtractTextResult {
        text: extracted_text,
        page_count,
    })
}

// ============================================================================
// REMOVE PASSWORD (Şifre/Kilit Kaldır)
// ============================================================================
#[derive(Deserialize)]
pub struct RemovePasswordRequest {
    pub file_path: String,
    pub output_path: String,
    pub password: String,
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_remove_password(request: RemovePasswordRequest) -> Result<String, String> {
    // TODO: Implement password removal
    // Decrypt PDF with provided password and save without encryption
    
    Ok(format!(
        "Remove Password: {} would be decrypted and saved to {}",
        request.file_path, request.output_path
    ))
}

// ============================================================================
// PROTECT PDF (Şifrele)
// ============================================================================
#[derive(Deserialize)]
pub struct ProtectRequest {
    pub file_path: String,
    pub output_path: String,
    pub user_password: Option<String>,  // Password to open
    pub owner_password: Option<String>, // Password to modify
    pub permissions: ProtectPermissions,
}

#[derive(Deserialize)]
pub struct ProtectPermissions {
    pub allow_printing: bool,
    pub allow_copying: bool,
    pub allow_modification: bool,
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_protect(request: ProtectRequest) -> Result<String, String> {
    // TODO: Implement PDF encryption
    // Add user/owner passwords and set permissions
    
    let mut features = Vec::new();
    if request.user_password.is_some() {
        features.push("user password");
    }
    if request.owner_password.is_some() {
        features.push("owner password");
    }
    
    Ok(format!(
        "Protect PDF: {} would be encrypted with {}",
        request.file_path,
        features.join(" and ")
    ))
}

// ============================================================================
// WATERMARK (Filigran Ekle)
// ============================================================================
#[derive(Deserialize)]
pub struct WatermarkRequest {
    pub file_path: String,
    pub output_path: String,
    pub text: String,
    pub opacity: f32,     // 0.0 to 1.0
    pub rotation: i32,    // degrees
    pub font_size: u32,
    pub color: String,    // hex color like "#FF0000"
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_watermark(request: WatermarkRequest) -> Result<String, String> {
    // TODO: Implement watermark addition
    // Overlay text on each page with specified styling
    
    Ok(format!(
        "Watermark: '{}' would be added to {} with {}% opacity",
        request.text,
        request.file_path,
        (request.opacity * 100.0) as u32
    ))
}

// ============================================================================
// METADATA EDITOR (Künye Düzenleyici)
// ============================================================================
#[derive(Deserialize, Serialize)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
}

#[derive(Deserialize)]
pub struct MetadataRequest {
    pub file_path: String,
    pub output_path: Option<String>, // If None, just read metadata
    pub metadata: Option<PdfMetadata>,
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_metadata(request: MetadataRequest) -> Result<PdfMetadata, String> {
    // Simplified placeholder - full implementation needs more complex lopdf handling
    if let Some(new_metadata) = request.metadata {
        // Write mode - would save metadata here
        Ok(new_metadata)
    } else {
        // Read mode - would read metadata here
        Ok(PdfMetadata {
            title: Some("Document Title".to_string()),
            author: Some("Author Name".to_string()),
            subject: None,
            keywords: None,
            creator: None,
            producer: None,
        })
    }
}
