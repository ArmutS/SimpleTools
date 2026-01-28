# SimpleTools - Module Planning

This file contains the detailed planning for the 12 modules and 12 functions within each module in the SimpleTools project.

## Module 1: Text Tools ✅ (COMPLETED)

**Shortcut**: T
**Status**: Ready
**Backend**: `src-tauri/src/textfunc.rs`

### Functions (12/12)

1. **Regex Tester** (R) ✅
   - Test regex patterns
   - Match highlighting
   - Show capture groups

2. **Text Diff** (D) ✅
   - Compare two texts
   - Side-by-side view
   - Highlight differences

3. **String Tools** (S) ✅
   - Case conversions (upper, lower, title, camel, snake, kebab)
   - String manipulations

4. **JWT Decoder** (J) ✅
   - Decode JWT tokens
   - Show header, payload, signature
   - Validation

5. **Slug Generator** (L) ✅
   - Create URL-friendly slugs
   - Clean special characters

6. **Text Strip** (T) ✅
   - Clean whitespace
   - Normalize line endings

7. **Lorem Ipsum Generator** (I) - TO BE ADDED
   - Generate placeholder text
   - Paragraph/word count settings

8. **Base64 Encoder/Decoder** (B) - TO BE ADDED
   - Base64 encode/decode
   - File support

9. **URL Encoder/Decoder** (U) - TO BE ADDED
   - URL encode/decode
   - Query string parser

10. **Markdown Preview** (M) - TO BE ADDED
    - Markdown to HTML
    - Live preview

11. **Character Counter** (C) - TO BE ADDED
    - Character, word, line counts
    - Reading time calculation

12. **Text Sorter** (O) - TO BE ADDED
    - Sort alphabetically
    - Reverse, unique, shuffle

---

## Module 2: PDF Tools ✅ (COMPLETED)

**Shortcut**: P
**Status**: Ready
**Backend**: `src-tauri/src/pdffunc.rs`
**Note**: Hybrid architecture using `lopdf` (Native Rust) for structure/encryption and `pdfium-render` for rendering.

### Functions (12/12)

1. **PDF Merger** (M) ✅ **With Advanced Features**
   - Merges scattered PDF files into a single file
   - **Split View UI**: Controls on left panel, preview on right (Planned)
   - **Opaque Interface**: Solid (non-transparent) design for better readability
   - **Smart Defaults**: Automatic output path and filename generation
   - **File Metadata**: Display page count, file size
   - **Drag & Drop Sorting**: Sort files by dragging
   - **Encrypted File Detection**: Warning with 🔒 icon
   - **Success Feedback**: "Open Folder" and "Open File" buttons
   - Reason: Sending 5 separate attachments to a client looks unprofessional; a single file is requested
   - **Status**: ✅ Ready

2. **PDF Splitter** (S) ✅
   - Extracts only specific pages (e.g., 5-10) from a 100-page report into a new PDF
   - "Make each page a separate file" option
   - Reason: To avoid sending the entire file when "just send me the relevant page" is requested
   - **Tech**: Native Rust implementation (lightweight)

3. **Images to PDF** (I) ✅
   - Converts photos of documents (JPG/PNG) taken with a phone or scanned into a single PDF
   - Reason: Visa applications or HR documents usually require a single PDF, not photos

4. **PDF to Images** (G) ✅
   - Exports each page of a PDF as high-quality JPG or PNG
   - **Tech**: Uses `pdfium-render` for high-fidelity rendering
   - Reason: To take a single slide from a presentation and post it as an image on Instagram or WhatsApp

5. **Compress PDF** (C) ✅
   - Reduces a 20 MB file to 2-3 MB without significantly ruining quality
   - Reason: Email attachments often have a 25MB limit, or upload sites have size limits

6. **Rotate Pages** (R) ✅
   - Rotates incorrectly scanned pages 90/180 degrees and saves
   - Reason: To avoid neck strain trying to read a document that came out of the scanner upside down

7. **Delete Pages** (D) ✅
   - Removes selected pages from inside a PDF
   - Reason: To weed out bad pages without rescanning
   - **Tech**: Native Rust (preserves bookmarks/metadata)

8. **Extract Text** (T) ✅
   - Extracts selectable text inside a PDF as pure txt (not OCR, embedded text)
   - Reason: To get content when copy-pasting is disabled or difficult

9. **Remove Password** (U) ✅
   - Permanently removes the password from a file you know the password to
   - Owner Password removal
   - Reason: You don't want to enter a password every time you archive your own credit card statement

10. **Protect PDF** (P) ✅
    - Adds an open password (User Password) and permissions (Owner Password)
    - **Tech**: Native Rust Encryption (RC4 128-bit)
    - Reason: Security is mandatory when sending payroll or confidential project files

11. **Watermark** (W) ✅
    - Writes a semi-transparent watermark diagonally across pages
    - Reason: To indicate document status or prevent unauthorized use

12. **Metadata Editor** (E) ✅
    - Changes background info like Author, Title
    - Reason: You don't want someone else's name in the "Author" section when submitting homework/report you got from someone else

### Advanced Features (PDF Merger)

- ✅ **Split View UI**: Separation of controls and preview
- ✅ **Solid Background**: Non-transparent, focus-oriented design
- ✅ **Smart Defaults**: Automatic output path and filename generation
- ✅ **File Metadata**: Page count, file size, encryption status
- ✅ **Drag & Drop Sorting**: Sort files by bringing
- ✅ **Encrypted Detection**: Detect encrypted files and warn
- ✅ **Success Feedback**: Buttons to open folder/file
- ✅ **Error Handling**: Detailed error messages

---

## Module 3: Converters ⚠️

**Shortcut**: C
**Status**: ✅ All Implemented / ⚠️ **UNTESTED**
**Backend**: Multiple modules (`convertfunc.rs`, `datafunc.rs`, `archivefunc.rs`, `subtitlefunc.rs`, `imagefunc.rs`, `iconfunc.rs`, `fontfunc.rs`, `ebookfunc.rs`, `vectorfunc.rs`, `audiofunc.rs`, `videofunc.rs`, `cadfunc.rs`)
**Note**: All 11 converters have working backends and UIs, but **NONE have been tested**.
**Workflow**: All functions follow the same pattern:

1. **Upload**: Drag & drop or select file(s) (Auto-detect extension)
2. **Select Target**: Checkbox list of available "Convert To" formats (Multi-select allowed)
3. **Paths**: Select output directory
4. **Action**: Convert

### Functions (12/12)

1. **Office Converter** (O) ✅ ⚠️ **UNTESTED**
   - Inputs: DOCX, DOC, PDF
   - Targets: PDF, HTML, Text
   - **Status**: Worker-based conversion implemented
   - **Note**: DOCX→PDF/HTML working, needs testing

2. **Image Converter** (I) ✅ ⚠️ **UNTESTED**
   - Inputs: JPG, PNG, WebP, GIF, TIFF, BMP
   - Targets: JPG, PNG, WebP, GIF, BMP, TIFF
   - **Backend**: `imagefunc.rs` - Pure Rust with `image` crate
   - **Features**: Quality settings, resize function

3. **Video Converter** (V) ✅ ⚠️ **UNTESTED - PLACEHOLDER**
   - Inputs: MP4, AVI, MKV, WebM
   - Targets: MP4 (placeholder only)
   - **Backend**: `videofunc.rs` - File copy only
   - **Note**: Needs external tool or WASM for real conversion

4. **Audio Converter** (A) ✅ ⚠️ **UNTESTED - PLACEHOLDER**
   - Inputs: MP3, WAV, OGG, FLAC
   - Targets: MP3, WAV, OGG, FLAC (placeholder)
   - **Backend**: `audiofunc.rs` - File copy only
   - **Note**: Needs codec libraries for real conversion

5. **Archive Converter** (Z) ✅ ⚠️ **UNTESTED**
   - Inputs: ZIP, TAR, TAR.GZ, GZ
   - Targets: ZIP, TAR, TAR.GZ
   - **Backend**: `archivefunc.rs` - Pure Rust with `zip`, `tar`, `flate2`
   - **Features**: Extract → Re-pack workflow

6. **E-Book Converter** (E) ✅ ⚠️ **UNTESTED - PARTIAL**
   - Inputs: TXT, EPUB, MOBI, PDF (limited)
   - Targets: TXT (working), others placeholder
   - **Backend**: `ebookfunc.rs` - Text extraction only
   - **Note**: TXT only, EPUB/MOBI need parsing

7. **Font Converter** (F) ✅ ⚠️ **UNTESTED - PLACEHOLDER**
   - Inputs: TTF, OTF, WOFF, WOFF2
   - Targets: TTF, OTF, WOFF, WOFF2 (placeholder)
   - **Backend**: `fontfunc.rs` - File copy only
   - **Note**: Needs font parsing/encoding libraries

8. **Data Converter** (D) ✅ ⚠️ **UNTESTED**
   - Inputs: JSON, XML, YAML, CSV, TOML
   - Targets: JSON, XML, YAML, CSV, TOML
   - **Backend**: `datafunc.rs` - Pure Rust with `serde` family
   - **Features**: Bidirectional conversion, smart format detection

9. **Vector Converter** (S) ✅ ⚠️ **UNTESTED - PLACEHOLDER**
   - Inputs: SVG, PDF, EPS
   - Targets: SVG (copy only)
   - **Backend**: `vectorfunc.rs` - File copy only
   - **Note**: Needs SVG rendering library

10. **3D/CAD Converter** (M) ✅ ⚠️ **UNTESTED - PLACEHOLDER**
    - Inputs: STL, OBJ, GLTF
    - Targets: STL, OBJ, GLTF (placeholder)
    - **Backend**: `cadfunc.rs` - File copy only
    - **Note**: Needs 3D parsing libraries

11. **Icon Converter** (C) ✅ ⚠️ **UNTESTED**
    - Inputs: PNG, JPG, SVG, ICO
    - Targets: ICO (multi-size), ICNS (limited), PNG
    - **Backend**: `iconfunc.rs` - Custom ICO encoder
    - **Features**: Multi-size ICO generation

12. **Subtitle Converter** (T) ✅ ⚠️ **UNTESTED**
    - Inputs: SRT, VTT, ASS, SSA
    - Targets: SRT, VTT, ASS, SSA
    - **Backend**: `subtitlefunc.rs` - Pure Rust text parsing
    - **Features**: Preserves timing, handles multi-line text

---

## Module 4: File & System 🚧

**Shortcut**: F
**Status**: Planned
**Backend**: `src-tauri/src/filefunc.rs` (to be created)

### Functions (0/12)

1. **Hash Generator** (H)
   - MD5, SHA1, SHA256, SHA512
   - File hashing

2. **File Renamer** (R)
   - Batch file renaming
   - Regex pattern support

3. **Duplicate Finder** (D)
   - Find duplicate files
   - Hash-based comparison

4. **Disk Usage Analyzer** (U)
   - Folder size analysis
   - Visual graph

5. **File Splitter** (S)
   - Split large files
   - Merge

6. **Checksum Verifier** (C)
   - Verify checksum
   - Multiple algorithms

7. **File Permissions** (P)
   - View file permissions
   - Chmod calculator

8. **Directory Tree** (T)
   - Visualize folder structure
   - Export to text

9. **File Watcher** (W)
   - Monitor file changes
   - Real-time monitoring

10. **Temp File Cleaner** (L)
    - Clean temporary files
    - Safe delete

11. **File Metadata Viewer** (M)
    - EXIF, ID3 tags
    - Edit metadata

12. **Symbolic Link Manager** (Y)
    - Create symlink
    - Manage links

---

## Module 5: Image Tools 🚧

**Shortcut**: I
**Status**: Planned
**Backend**: `src-tauri/src/imagefunc.rs` (to be created)

### Functions (0/12)

1. **Image Resizer** (R)
   - Resize
   - Maintain aspect ratio

2. **Image Compressor** (C)
   - Lossy/lossless compression
   - Quality setting

3. **Image Cropper** (O)
   - Crop tool
   - Preset dimensions

4. **Image Filters** (F)
   - Grayscale, blur, sharpen
   - Multiple filters

5. **Watermark Tool** (W)
   - Image/text watermark
   - Position setting

6. **Background Remover** (B)
   - Remove background
   - AI-powered

7. **Image to Base64** (A)
   - Base64 encode
   - Create Data URI

8. **QR Code Generator** (Q)
   - Generate QR code
   - Customization

9. **Barcode Generator** (D)
   - Generate barcode
   - Multiple formats

10. **Image Metadata Editor** (M)
    - Edit EXIF
    - Clean metadata

11. **Image Collage** (L)
    - Create collage
    - Layout options

12. **Screenshot Tool** (S)
    - Screenshot
    - Annotation

---

## Module 6: Network 🚧

**Shortcut**: N
**Status**: Planned
**Backend**: `src-tauri/src/networkfunc.rs` (to be created)

### Functions (0/12)

1. **IP Info** (I)
   - View IP info
   - Geolocation

2. **Port Scanner** (P)
   - Scan ports
   - Service detection

3. **DNS Lookup** (D)
   - DNS queries
   - A, MX, TXT records

4. **Ping Tool** (G)
   - Ping test
   - Latency measurement

5. **Traceroute** (T)
   - Trace route
   - Hop analysis

6. **WHOIS Lookup** (W)
   - Domain info
   - Registrar info

7. **SSL Certificate Checker** (S)
   - Check SSL certificate
   - Expiry date

8. **HTTP Headers** (H)
   - View HTTP headers
   - Request/response

9. **URL Shortener** (U)
   - Shorten URL
   - Custom alias

10. **Network Speed Test** (E)
    - Download/upload speed
    - Latency test

11. **MAC Address Lookup** (M)
    - MAC vendor lookup
    - OUI database

12. **Subnet Calculator** (C)
    - Calculate CIDR
    - IP range

---

## Module 7: Quick Commands 🚧

**Shortcut**: Q
**Status**: Planned
**Backend**: `src-tauri/src/quickfunc.rs` (to be created)

### Functions (0/12)

1. **UUID Generator** (U)
   - Create UUID v4
   - Batch generation

2. **Password Generator** (P)
   - Create strong password
   - Customizable

3. **Random Number** (N)
   - Generate random number
   - Range selection

4. **Lorem Ipsum** (L)
   - Placeholder text
   - Paragraph/word

5. **Cron Expression** (C)
   - Create cron expression
   - Human readable

6. **Epoch Converter** (E)
   - Unix timestamp
   - Date conversion

7. **JSON Formatter** (J)
   - JSON pretty print
   - Minify

8. **SQL Formatter** (S)
   - SQL formatting
   - Syntax highlight

9. **Regex Tester** (R)
   - Quick regex test
   - Common patterns

10. **Color Picker** (O)
    - Color selection
    - Format export

11. **ASCII Art** (A)
    - Text to ASCII art
    - Font selection

12. **Emoji Picker** (M)
    - Emoji search
    - Copy to clipboard

---

## Module 8: Dev Tools 🚧

**Shortcut**: D
**Status**: Planned
**Backend**: `src-tauri/src/devfunc.rs` (to be created)

### Functions (0/12)

1. **JSON Validator** (J)
   - JSON syntax check
   - Error highlighting

2. **XML Validator** (X)
   - XML syntax check
   - Schema validation

3. **YAML Validator** (Y)
   - YAML syntax check
   - Linting

4. **HTML Formatter** (H)
   - HTML beautify
   - Minify

5. **CSS Formatter** (C)
   - CSS beautify
   - Autoprefixer

6. **JavaScript Minifier** (M)
   - JS minification
   - Uglify

7. **Git Diff Viewer** (G)
   - View diff
   - Syntax highlight

8. **API Tester** (A)
   - Test REST API
   - Request builder

9. **GraphQL Playground** (Q)
   - Test GraphQL query
   - Schema explorer

10. **WebSocket Tester** (W)
    - WebSocket connection
    - Message testing

11. **Code Snippet Manager** (S)
    - Store snippets
    - Syntax highlight

12. **Regex Builder** (R)
    - Visual regex builder
    - Pattern library

---

## Modules 9-12: TBD 🚧

Ideas for future modules:

### Potential Modules

- **Security Tools**: Encryption, hashing, password audit
- **Database Tools**: Query builder, schema designer
- **Git Tools**: Commit helper, branch manager
- **Math Tools**: Calculator, equation solver
- **Data Tools**: Data generator, faker
- **Crypto Tools**: Cryptocurrency converter, wallet
- **Time Tools**: Timer, stopwatch, pomodoro
- **Note Tools**: Quick notes, markdown editor

---

## Progress Summary

| Module         | Status | Completed  | Remaining |
| -------------- | ------ | ---------- | --------- |
| Text Tools     | ✅     | 6/12       | 6         |
| PDF Tools      | ✅     | 12/12      | 0         |
| Converters     | ⚠️     | 11/12 ⚠️   | 1         |
| File & System  | 🚧     | 0/12       | 12        |
| Image Tools    | 🚧     | 0/12       | 12        |
| Network        | 🚧     | 0/12       | 12        |
| Quick Commands | 🚧     | 0/12       | 12        |
| Dev Tools      | 🚧     | 0/12       | 12        |
| TBD 9          | 🚧     | 0/12       | 12        |
| TBD 10         | 🚧     | 0/12       | 12        |
| TBD 11         | 🚧     | 0/12       | 12        |
| TBD 12         | 🚧     | 0/12       | 12        |
| **TOTAL**      |        | **29/144** | **115**   |

> [!WARNING]
> **Converters Module**: All 11 converters implemented but **NONE tested**. Testing required before production use.

**Progress**: 20.1%
