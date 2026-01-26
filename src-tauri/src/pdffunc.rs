use lopdf::Document;
use serde::{Deserialize, Serialize};
use std::fs;
use image::GenericImageView;
use pdfium_render::prelude::*;
use std::path::Path;
use tauri::Manager;
use tauri::path::BaseDirectory;

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

fn load_pdf_document(path: &str) -> Result<Document, String> {
    Document::load(path).map_err(|e| {
        let msg = e.to_string();
        if msg.contains("Invalid file trailer") || msg.contains("failed parsing cross reference table") {
            "Failed to load PDF: The file appears to be corrupted or invalid (Invalid file trailer).".to_string()
        } else {
            format!("Failed to load PDF: {}", msg)
        }
    })
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
    // Try to load PDF
    match load_pdf_document(&file_path) {
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
            error: Some(e),
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

    // Load all documents
    let mut documents = Vec::new();
    for path in &request.files {
        let doc = load_pdf_document(path)?;
        documents.push(doc);
    }

    // Take the first document as the base
    let mut base_doc = documents.remove(0);
    
    // Disable compression for simplicity during manipulation
    base_doc.version = "1.5".to_string();

    // Get the Pages object ID from the base document
    let base_catalog_id = base_doc.trailer.get(b"Root")
        .map_err(|_| "Base PDF missing Root".to_string())?
        .as_reference()
        .map_err(|_| "Base PDF Root is not a reference".to_string())?;
        
    let base_pages_id = base_doc.get_object(base_catalog_id)
        .and_then(|obj| obj.as_dict())
        .and_then(|dict| dict.get(b"Pages"))
        .map_err(|_| "Base PDF missing Pages".to_string())?
        .as_reference()
        .map_err(|_| "Base PDF Pages is not a reference".to_string())?;

    for mut doc in documents {
        // Renumber objects in the incoming document to avoid collisions
        // We start renumbering after the current max_id of base_doc
        doc.renumber_objects_with(base_doc.max_id + 1);
        
        // Update base_doc.max_id to reflect the new range
        base_doc.max_id = doc.max_id;

        // Get the list of pages from the incoming document
        let pages = doc.get_pages()
            .into_values()
            .collect::<Vec<_>>();

        // Append explicit objects from doc to base_doc
        // (This includes the renumbered pages and their resources)
        base_doc.objects.extend(doc.objects);

        // Add the new page references to the base document's Pages tree
        if let Ok(pages_object) = base_doc.get_object_mut(base_pages_id) {
            if let Ok(pages_dict) = pages_object.as_dict_mut() {
                // Add Kids
                if let Ok(kids) = pages_dict.get_mut(b"Kids").and_then(|k| k.as_array_mut()) {
                    for page_id in &pages {
                        kids.push(lopdf::Object::Reference(*page_id));
                    }
                }
                
                // Update Count
                if let Ok(count) = pages_dict.get(b"Count").and_then(|c| c.as_i64()) {
                    pages_dict.set(b"Count", count + pages.len() as i64);
                }
            }
        }
    }

    // Prune unused objects (optional but good for clean output)
    base_doc.prune_objects();
    
    // Compress and Save
    base_doc.compress();
    base_doc
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
    let doc = load_pdf_document(&request.file_path)?;
    let pages = doc.get_pages();
    let total_pages = pages.len() as u32;

    match request.mode.as_str() {
        "range" => {
            let start = request.start_page.unwrap_or(1).max(1);
            let end = request.end_page.unwrap_or(total_pages).min(total_pages);

            if start > end {
                return Err(format!("Invalid page range: {}-{}", start, end));
            }

            let mut new_doc = doc.clone();
            
            // Keep keys where start <= page_num <= end
            let pages_to_keep: Vec<u32> = (start..=end).collect();
            
            // Helper to modify document to retain only specific pages
            retain_pages(&mut new_doc, &pages_to_keep)?;

            let output_path = format!("{}/pages_{}-{}.pdf", request.output_dir, start, end);
            new_doc.save(&output_path)
                .map_err(|e| format!("Failed to save: {}", e))?;

            Ok(format!("Extracted pages {}-{} to {}", start, end, output_path))
        }
        "individual" => {
            let mut saved_count = 0;
            // For individual, we clone the original doc for each page to ensure isolation
            // Optimization: If PDF is huge, this is slow. But for desktop app it is usually accceptable.
            
            for i in 1..=total_pages {
                let mut new_doc = doc.clone();
                retain_pages(&mut new_doc, &[i])?;
                
                let output_path = format!("{}/page_{}.pdf", request.output_dir, i);
                new_doc.save(&output_path)
                    .map_err(|e| format!("Failed to save page {}: {}", i, e))?;
                saved_count += 1;
            }
            Ok(format!("Split {} pages into separate files in {}", saved_count, request.output_dir))
        }
        _ => Err("Invalid split mode. Use 'range' or 'individual'".to_string()),
    }
}

// Helper to retain only specific pages in a document
fn retain_pages(doc: &mut Document, pages_to_keep: &[u32]) -> Result<(), String> {
    // 1. Get current pages mapping
    let pages = doc.get_pages();
    let mut kept_object_ids = Vec::new();
    
    // 2. Identify ObjectIds to keep
    for page_num in pages_to_keep {
        if let Some(id) = pages.get(page_num) {
            kept_object_ids.push(*id);
        }
    }
    
    if kept_object_ids.is_empty() {
        return Err("No pages selected to keep".to_string());
    }

    // 3. Create a new Pages dictionary (flattened)
    let pages_root_id = doc.new_object_id();
    let kids: Vec<lopdf::Object> = kept_object_ids.iter()
        .map(|&id| lopdf::Object::Reference(id))
        .collect();

    let pages_dict = lopdf::Dictionary::from_iter(vec![
        ("Type", "Pages".into()),
        ("Count", (kids.len() as i32).into()),
        ("Kids", kids.into()),
    ]);

    doc.objects.insert(pages_root_id, lopdf::Object::Dictionary(pages_dict));

    // 4. Update Catalog to point to new Pages root
    let catalog_id = doc.trailer.get(b"Root")
        .map_err(|_| "Missing Root in trailer".to_string())?
        .as_reference()
        .map_err(|_| "Root is not a reference".to_string())?;

    if let Ok(catalog) = doc.get_object_mut(catalog_id).and_then(|o| o.as_dict_mut()) {
        catalog.set("Pages", lopdf::Object::Reference(pages_root_id));
    } else {
        return Err("Failed to access Catalog dictionary".to_string());
    }

    // 5. Prune unused objects (removes old Page Tree nodes and unreferenced pages)
    doc.prune_objects();
    
    // 6. Compress for efficiency
    doc.compress();
    
    Ok(())
}

// ============================================================================
// IMAGES TO PDF (Resimden PDF'e)
// ============================================================================
#[derive(Deserialize)]
pub struct ImagesToPdfRequest {
    pub image_paths: Vec<String>,
    pub output_path: String,
    pub _page_size: Option<String>, // "A4", "Letter", etc.
}

#[tauri::command(rename_all = "snake_case")]

pub fn images_to_pdf(request: ImagesToPdfRequest) -> Result<String, String> {
    if request.image_paths.is_empty() {
        return Err("No images provided".to_string());
    }

    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let mut page_ids = Vec::new();

    for path in &request.image_paths {
        let img = image::open(path)
            .map_err(|e| format!("Failed to open image {}: {}", path, e))?;
        
        let (width, height) = img.dimensions();
        let color_type = img.color();
        let bits_per_component = 8; // Assuming 8-bit per channel for simplicity

        let mut dict = lopdf::Dictionary::new();
        dict.set("Type", lopdf::Object::Name(b"XObject".to_vec()));
        dict.set("Subtype", lopdf::Object::Name(b"Image".to_vec()));
        dict.set("Width", width as i64);
        dict.set("Height", height as i64);
        dict.set("BitsPerComponent", bits_per_component);
        
        let color_space = match color_type {
            image::ColorType::L8 => b"DeviceGray".to_vec(),
            image::ColorType::Rgb8 => b"DeviceRGB".to_vec(),
            image::ColorType::Rgba8 => b"DeviceRGB".to_vec(), // Alpha strip needed? For now treat as RGB or simple
            _ => b"DeviceRGB".to_vec(),
        };
        dict.set("ColorSpace", lopdf::Object::Name(color_space));
        
        // Simple raw bytes - for production might want to use specific encoding like DCTDecode (JPEG) if source is JPEG
        // For generic implementation, we'll convert to raw RGB/Gray bytes
        let img_data = match color_type {
             image::ColorType::Rgba8 => {
                 // Strip alpha channel for PDF compatibility (simple approach)
                 img.to_rgb8().into_raw()
             },
             _ => img.into_bytes(),
        };

        let stream = lopdf::Stream::new(dict, img_data);
        let xobject_id = doc.add_object(stream);

        // Define page content to place the image
        // Scale image to page size or use image size
        // For simplicity: Use image size as page size
        let content = lopdf::content::Content {
            operations: vec![
                lopdf::content::Operation::new("q", vec![]),
                lopdf::content::Operation::new("cm", vec![
                    (width as f32).into(), 0.into(),
                    0.into(), (height as f32).into(),
                    0.into(), 0.into()
                ]),
                lopdf::content::Operation::new("Do", vec![lopdf::Object::Name(b"Im0".to_vec())]),
                lopdf::content::Operation::new("Q", vec![]),
            ],
        };

        let content_id = doc.add_object(lopdf::Stream::new(lopdf::Dictionary::new(), content.encode().unwrap()));

        let page_id = doc.add_object(lopdf::Dictionary::from_iter(vec![
            ("Type", "Page".into()),
            ("Parent", pages_id.into()),
            ("MediaBox", vec![0.into(), 0.into(), (width as f32).into(), (height as f32).into()].into()),
            ("Contents", content_id.into()),
            ("Resources", lopdf::Dictionary::from_iter(vec![
                ("XObject", lopdf::Dictionary::from_iter(vec![
                    ("Im0", xobject_id.into())
                ]).into())
            ]).into()),
        ]));

        page_ids.push(lopdf::Object::Reference(page_id));
    }

    let pages = lopdf::Dictionary::from_iter(vec![
        ("Type", "Pages".into()),
        ("Count", (page_ids.len() as i32).into()),
        ("Kids", page_ids.into()),
    ]);

    doc.objects.insert(pages_id, lopdf::Object::Dictionary(pages));
    
    let catalog_id = doc.add_object(lopdf::Dictionary::from_iter(vec![
        ("Type", "Catalog".into()),
        ("Pages", pages_id.into()),
    ]));

    doc.trailer.set("Root", catalog_id);

    doc.save(&request.output_path)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;

    Ok(format!(
        "Successfully converted {} images to {}",
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
pub fn pdf_to_images(app: tauri::AppHandle, request: PdfToImagesRequest) -> Result<String, String> {
    // Combine file path and potential platform-specific library search paths
    // For Linux, we might need libpdfium.so. On Arch, `pdfium-binaries` or `libpdfium-nojs` provides it.
    // If dynamic binding fails, we return an error instructing to install the lib.
    
    let pdfium_lib_path = app
        .path()
        .resolve("libs/libpdfium.so", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    println!("DEBUG: Resolved Pdfium Path: {:?}", pdfium_lib_path);
    if !pdfium_lib_path.exists() {
        println!("DEBUG: Pdfium library NOT FOUND at resolved path!");
    } else {
        println!("DEBUG: Pdfium library FOUND at resolved path.");
    }

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(pdfium_lib_path.to_str().unwrap())
        .map_err(|e| {
            println!("DEBUG: Failed to bind to bundled Pdfium: {}", e);
            e
        })
        .or_else(|original_err| {
            println!("DEBUG: Attempting to bind to system library...");
            Pdfium::bind_to_system_library()
            .map_err(|e| format!("Failed to bind to Pdfium library (bundled error: {}, system error: {})", original_err, e))
        })?,
    );
    
    let document = pdfium.load_pdf_from_file(&request.file_path, None)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;

    let output_dir = Path::new(&request.output_dir);
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
    }

    let scale = request.dpi.map(|dpi| dpi as f32 / 72.0).unwrap_or(2.0); // 72 dpi is 1.0, so 144 is 2.0
    // let render_config = PdfRenderConfig::new()... (unused)

    let mut saved_count = 0;
    
    for (i, page) in document.pages().iter().enumerate() {
        // We can optionally set width/height per page if they vary
        let width = (page.width().value * scale) as i32;
        let height = (page.height().value * scale) as i32;
        
        let bitmap = page.render(width, height, Some(PdfPageRenderRotation::None))
            .map_err(|e| format!("Failed to render page {}: {}", i, e))?;
            
        let image = bitmap.as_image(); // Returns DynamicImage
        
        let ext = if request.format == "jpg" || request.format == "jpeg" { "jpg" } else { "png" };
        let output_path = output_dir.join(format!("page_{}.{}", i + 1, ext));
        
        let format = if ext == "jpg" { image::ImageFormat::Jpeg } else { image::ImageFormat::Png };
        
        image.save_with_format(&output_path, format)
            .map_err(|e| format!("Failed to save image {}: {}", i, e))?;
            
        saved_count += 1;
    }

    Ok(format!("Converted {} pages to images in {}", saved_count, request.output_dir))
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
    let mut doc = load_pdf_document(&request.file_path)?;

    doc.compress();
    
    // Prune unused objects to further reduce size
    doc.prune_objects();

    // Variable quality could be implemented by re-compressing images with lower quality
    // But lopdf doesn't support image re-encoding out of the box easily.
    // For now, we just ensure streams are compressed and unused objects are removed.
    
    doc.save(&request.output_path)
        .map_err(|e| format!("Failed to save compressed PDF: {}", e))?;

    let quality = request.quality.unwrap_or_else(|| "medium".to_string());
    Ok(format!(
        "PDF Compressed ({} quality) to {}",
        quality, request.output_path
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
    let mut doc = load_pdf_document(&request.file_path)?;

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
    pub pages_to_delete: Vec<u32>, // 1-based page numbers
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_delete_pages(request: DeletePagesRequest) -> Result<String, String> {
    if request.pages_to_delete.is_empty() {
        return Err("No pages specified for deletion".to_string());
    }

    let mut doc = load_pdf_document(&request.file_path)?;
    let pages = doc.get_pages();
    let total_pages = pages.len() as u32;
    
    // Identify pages to keep
    let mut pages_to_keep = Vec::new();
    for i in 1..=total_pages {
        if !request.pages_to_delete.contains(&i) {
            pages_to_keep.push(i);
        }
    }

    if pages_to_keep.is_empty() {
         return Err("Resulting PDF would be empty (all pages deleted)".to_string());
    }

    // Use our helper to rewrite the page tree
    retain_pages(&mut doc, &pages_to_keep)?;

    doc.save(&request.output_path)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;

    Ok(format!(
        "Deleted {} pages. Saved to {}",
        request.pages_to_delete.len(),
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
    let doc = load_pdf_document(&request.file_path)?;

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
    let mut doc = load_pdf_document(&request.file_path)?;

    if doc.is_encrypted() {
        doc.decrypt(&request.password)
            .map_err(|e| format!("Failed to decrypt PDF (wrong password?): {}", e))?;
    }
    
    // Decryption removes the Encrypt dictionary from trailer internally usually,
    // or we might need to ensure it's saved without encryption.
    // lopdf's decrypt should handle state.
    
    doc.save(&request.output_path)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;

    Ok(format!(
        "Password removed. Saved to {}",
        request.output_path
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
    let mut doc = load_pdf_document(&request.file_path)?;

    let user_pwd = request.user_password.unwrap_or_default();
    let owner_pwd = request.owner_password.unwrap_or_default();

    if user_pwd.is_empty() && owner_pwd.is_empty() {
         return Err("At least one password (user or owner) must be provided".to_string());
    }

    // Implement encryption using lopdf
    use lopdf::encryption::{EncryptionState, EncryptionVersion, Permissions};
    use std::convert::TryFrom;

    // Default permissions: Allow everything mostly
    let permissions = Permissions::PRINTABLE | Permissions::MODIFIABLE | Permissions::COPYABLE | Permissions::ANNOTABLE | Permissions::FILLABLE | Permissions::ASSEMBLABLE | Permissions::PRINTABLE_IN_HIGH_QUALITY;

    // Adjust permissions based on request (masking out if disabled)
    let mut final_permissions = permissions;
    if !request.permissions.allow_printing {
        final_permissions.remove(Permissions::PRINTABLE | Permissions::PRINTABLE_IN_HIGH_QUALITY);
    }
    if !request.permissions.allow_modification {
        final_permissions.remove(Permissions::MODIFIABLE | Permissions::ANNOTABLE | Permissions::FILLABLE | Permissions::ASSEMBLABLE);
    }
    if !request.permissions.allow_copying {
        final_permissions.remove(Permissions::COPYABLE | Permissions::COPYABLE_FOR_ACCESSIBILITY);
    }

    // Use V2 (RC4 128-bit)
    let version = EncryptionVersion::V2 {
        document: &doc,
        owner_password: &owner_pwd,
        user_password: &user_pwd,
        key_length: 128, 
        permissions: final_permissions,
    };
    
    let state = EncryptionState::try_from(version)
        .map_err(|e| format!("Failed to create encryption state: {}", e))?;
    
    doc.encrypt(&state)
        .map_err(|e| format!("Failed to encrypt PDF: {}", e))?;

    doc.save(&request.output_path)
        .map_err(|e| format!("Failed to save encrypted PDF: {}", e))?;

    Ok(format!(
        "PDF Protected and saved to {}",
        request.output_path
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
    pub _font_size: u32,
    pub _color: String,    // hex color like "#FF0000"
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_watermark(request: WatermarkRequest) -> Result<String, String> {
    let mut doc = load_pdf_document(&request.file_path)?;

    let text = &request.text;
    let rotation = request.rotation as f32;
    let opacity = request.opacity;
    
    let angle = rotation.to_radians();
    let c = angle.cos();
    let s = angle.sin();
    
    let page_ids: Vec<_> = doc.page_iter().collect();
    
    // Define the font we want to use (Standard Helvetica)
    // We must ensure the Font dictionary exists in Resources
    let font_ref = doc.add_object(lopdf::Dictionary::from_iter(vec![
        ("Type", "Font".into()),
        ("Subtype", "Type1".into()),
        ("BaseFont", "Helvetica".into()),
    ]));

    for page_id in page_ids {
        // Add Font to Resources
        if let Ok(page) = doc.get_object_mut(page_id) {
            if let Ok(dict) = page.as_dict_mut() {
                if let Ok(resources) = dict.get_mut(b"Resources").and_then(|o| o.as_dict_mut()) {
                    if let Ok(fonts) = resources.get_mut(b"Font").and_then(|o| o.as_dict_mut()) {
                        fonts.set("F1", lopdf::Object::Reference(font_ref));
                    } else {
                        // Create Font dict if missing
                        let mut fonts = lopdf::Dictionary::new();
                        fonts.set("F1", lopdf::Object::Reference(font_ref));
                        resources.set("Font", fonts);
                    }
                } else {
                    // Create Resources if missing
                    let mut fonts = lopdf::Dictionary::new();
                    fonts.set("F1", lopdf::Object::Reference(font_ref));
                    let mut resources = lopdf::Dictionary::new();
                    resources.set("Font", fonts);
                    dict.set("Resources", resources);
                }
            }
        }

        // Add Content Stream
        let _ = doc.add_to_page_content(page_id,  
             lopdf::content::Content {
                 operations: vec![
                     lopdf::content::Operation::new("q", vec![]),
                     lopdf::content::Operation::new("G", vec![((1.0 - opacity)).into()]), 
                     lopdf::content::Operation::new("g", vec![((1.0 - opacity)).into()]),
                     lopdf::content::Operation::new("BT", vec![]),
                     lopdf::content::Operation::new("Tf", vec!["F1".into(), 48.into()]),
                     lopdf::content::Operation::new("Tm", vec![
                         c.into(), s.into(), (-s).into(), c.into(), 
                         250.into(), 400.into() // Approx center for typical A4
                     ]),
                     lopdf::content::Operation::new("Tj", vec![lopdf::Object::String(text.as_bytes().to_vec(), lopdf::StringFormat::Literal)]),
                     lopdf::content::Operation::new("ET", vec![]),
                     lopdf::content::Operation::new("Q", vec![])
                 ]
             }
        );
    }
    
    doc.save(&request.output_path)
         .map_err(|e| format!("Failed to save PDF: {}", e))?;
    
    Ok(format!(
        "Watermark added to {}",
        request.output_path
    ))
}

// ============================================================================
// METADATA EDITOR (Künye Düzenleyici)
#[derive(Deserialize, Serialize)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MetadataRequest {
    pub file_path: String,
    pub output_path: String,
    pub metadata: PdfMetadata,
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_read_metadata(path: String) -> Result<PdfMetadata, String> {
    let doc = load_pdf_document(&path)?;
        
    // data is Option<&Dictionary>
    let info = doc.trailer.get(b"Info")
        .and_then(|id| doc.get_object(id.as_reference().unwrap()))
        .and_then(|obj| obj.as_dict())
        .map_err(|_| "Metadata not found".to_string());
        
    if let Ok(dict) = info {
        let get_str = |key: &[u8]| -> Option<String> {
            dict.get(key)
                .ok()
                .and_then(|o| o.as_str().ok())
                .map(|s| String::from_utf8_lossy(s).to_string())
        };
        
        Ok(PdfMetadata {
            title: get_str(b"Title"),
            author: get_str(b"Author"),
            subject: get_str(b"Subject"),
            keywords: get_str(b"Keywords"),
            creator: get_str(b"Creator"),
            producer: get_str(b"Producer"),
            creation_date: get_str(b"CreationDate"),
            modification_date: get_str(b"ModDate"),
        })
    } else {
        Ok(PdfMetadata {
            title: None,
            author: None,
            subject: None,
            keywords: None,
            creator: None,
            producer: None,
            creation_date: None,
            modification_date: None,
        })
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn pdf_metadata(request: MetadataRequest) -> Result<String, String> {
    let mut doc = load_pdf_document(&request.file_path)?;

    // Find or Create Info dictionary
    let info_id = match doc.trailer.get(b"Info") {
        Ok(obj) => obj.as_reference().unwrap(),
        Err(_) => {
            let dict = lopdf::Dictionary::new();
            let id = doc.add_object(dict);
            doc.trailer.set("Info", id);
            id
        }
    };
    
    // Update fields
    if let Ok(info_obj) = doc.get_object_mut(info_id) {
        if let Ok(dict) = info_obj.as_dict_mut() {
             let mut set_str = |key: &[u8], val: &Option<String>| {
                 if let Some(v) = val {
                     dict.set(key, lopdf::Object::String(v.as_bytes().to_vec(), lopdf::StringFormat::Literal));
                 }
             };
             
             set_str(b"Title", &request.metadata.title);
             set_str(b"Author", &request.metadata.author);
             set_str(b"Subject", &request.metadata.subject);
             set_str(b"Keywords", &request.metadata.keywords);
             set_str(b"Creator", &request.metadata.creator);
             set_str(b"Producer", &request.metadata.producer);
             set_str(b"CreationDate", &request.metadata.creation_date);
             set_str(b"ModDate", &request.metadata.modification_date);
        }
    }

    doc.save(&request.output_path)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;

    Ok(format!(
        "Metadata updated. Saved to {}",
        request.output_path
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_invalid_pdf_trailer() {
        // Create a dummy invalid PDF file
        let file_path = "invalid_test.pdf";
        let mut file = std::fs::File::create(file_path).expect("failed to create file");
        file.write_all(b"%PDF-1.5\nSome junk content\n%%EOF").expect("failed to write file");

        // Try to load it using get_pdf_info
        let result = get_pdf_info(file_path.to_string());
        println!("Result error: {:?}", result.error);
        
        // Try to load via Document::load directly to see exact error used in other functions
        let load_result = load_pdf_document(file_path);
        match load_result {
            Ok(_) => println!("Unexpected success loading invalid PDF"),
            Err(e) => println!("Document::load error: {}", e),
        }
        
        // Clean up
        let _ = std::fs::remove_file(file_path);

        // Assert that we get an error
        assert!(result.error.is_some());
        let err_msg = result.error.unwrap();
        // The exact error message might depend on lopdf version, but "context" usually adds "Cannot read PDF"
        assert!(err_msg.contains("The file appears to be corrupted or invalid"));
    }
}
