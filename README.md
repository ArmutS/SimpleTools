# SimpleTools

> Vim-benzeri klavye kısayolları ile hızlı erişim sağlayan modüler araç koleksiyonu

SimpleTools, geliştiriciler ve power user'lar için tasarlanmış, Alfred benzeri bir arayüze sahip masaüstü uygulamasıdır. `Alt+Space` ile hızlıca açılır ve vim-tarzı klavye kısayolları ile navigasyon sağlar.

## ✨ Özellikler

- 🚀 **Hızlı Erişim**: `Alt+Space` ile anında açılır
- ⌨️ **Vim-like Navigasyon**: Tek tuş kısayolları ile hızlı gezinme
- 🎯 **Modüler Yapı**: 12 farklı modül, her biri 12 fonksiyon (toplam 144 araç)
- 🖥️ **Çoklu Monitör Desteği**: Mouse pozisyonuna göre doğru monitörde açılır
- 🎨 **Tema Sistemi**: 5 farklı tema seçeneği
- ⚡ **Performanslı**: Rust backend ile hızlı işlem

## 🛠️ Teknoloji Stack

- **Frontend**: Svelte + TypeScript
- **Backend**: Rust (Tauri v2)
- **Build Tool**: Vite

## 📦 Kurulum

```bash
# Bağımlılıkları yükle
npm install

# Geliştirme modunda çalıştır
npm run tauri dev

# Production build
npm run tauri build
```

## 🎮 Kullanım

### Temel Navigasyon

1. **Uygulamayı Aç**: `Alt+Space`
2. **Modül Seç**: Tek tuş (örn: `T` = Text Tools)
3. **Fonksiyon Seç**: Tek tuş (örn: `R` = Regex Tester)
4. **Geri Dön**: `Esc`

### Örnek Kullanım

```
Alt+Space → T → R
```

Bu komut dizisi Regex Tester'ı açar.

## 📚 Modüller

### ✅ Text Tools (Hazır)

**Kısayol**: `T` | **İlerleme**: 6/12

- ✅ Regex Tester
- ✅ Text Diff
- ✅ String Tools
- ✅ JWT Decoder
- ✅ Slug Generator
- ✅ Text Strip
- 🚧 Lorem Ipsum Generator
- 🚧 Base64 Encoder/Decoder
- 🚧 URL Encoder/Decoder
- 🚧 Markdown Preview
- 🚧 Character Counter
- 🚧 Text Sorter

### 🚧 PDF Tools (Planlanıyor)

**Kısayol**: `P` | **İlerleme**: 10/12

- ✅ PDF Merger
- ✅ PDF Splitter
- ✅ Images to PDF
- 🚧 PDF to Images
- ✅ Compress PDF
- ✅ Rotate Pages
- ✅ Delete Pages
- ✅ Extract Text
- ✅ Remove Password
- 🚧 Protect PDF
- ✅ Watermark
- ✅ Metadata Editor

### 🚧 Converters (Planlanıyor)

**Kısayol**: `C` | **İlerleme**: 0/12

JSON↔YAML, CSV↔JSON, XML↔JSON, Color Converter, Unit Converter, Timestamp Converter, Number Base, Image Format, Audio Converter, Video Converter, Font Converter, Markdown to HTML

### 🚧 File & System (Planlanıyor)

**Kısayol**: `F` | **İlerleme**: 0/12

Hash Generator, File Renamer, Duplicate Finder, Disk Usage, File Splitter, Checksum Verifier, File Permissions, Directory Tree, File Watcher, Temp Cleaner, Metadata Viewer, Symlink Manager

### 🚧 Image Tools (Planlanıyor)

**Kısayol**: `I` | **İlerleme**: 0/12

Image Resizer, Compressor, Cropper, Filters, Watermark, Background Remover, Image to Base64, QR Code Generator, Barcode Generator, Metadata Editor, Image Collage, Screenshot Tool

### 🚧 Network (Planlanıyor)

**Kısayol**: `N` | **İlerleme**: 0/12

IP Info, Port Scanner, DNS Lookup, Ping, Traceroute, WHOIS, SSL Checker, HTTP Headers, URL Shortener, Speed Test, MAC Lookup, Subnet Calculator

### 🚧 Quick Commands (Planlanıyor)

**Kısayol**: `Q` | **İlerleme**: 0/12

UUID Generator, Password Generator, Random Number, Lorem Ipsum, Cron Expression, Epoch Converter, JSON Formatter, SQL Formatter, Regex Tester, Color Picker, ASCII Art, Emoji Picker

### 🚧 Dev Tools (Planlanıyor)

**Kısayol**: `D` | **İlerleme**: 0/12

JSON Validator, XML Validator, YAML Validator, HTML Formatter, CSS Formatter, JS Minifier, Git Diff Viewer, API Tester, GraphQL Playground, WebSocket Tester, Snippet Manager, Regex Builder

## ⌨️ Klavye Kısayolları

### Global Kısayollar

| Kısayol     | Aksiyon             |
| ----------- | ------------------- |
| `Alt+Space` | Uygulamayı aç/kapat |
| `Ctrl+G`    | Global listener     |
| `Esc`       | Pencere kapat       |

### Modül Kısayolları

| Tuş | Modül          | Durum |
| --- | -------------- | ----- |
| `T` | Text Tools     | ✅    |
| `P` | PDF Tools      | 🚧    |
| `C` | Converters     | 🚧    |
| `F` | File & System  | 🚧    |
| `I` | Image Tools    | 🚧    |
| `N` | Network        | 🚧    |
| `Q` | Quick Commands | 🚧    |
| `D` | Dev Tools      | 🚧    |

Detaylı kısayol listesi için [SHORTCUTS.md](SHORTCUTS.md) dosyasına bakın.

## 📖 Dokümantasyon

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Proje mimarisi ve teknik detaylar
- **[MODULES.md](MODULES.md)** - Tüm modüller ve fonksiyonların detaylı planlaması
- **[SHORTCUTS.md](SHORTCUTS.md)** - Klavye kısayolları referansı

## 🎯 Proje Durumu

**Toplam İlerleme**: 16/144 (%11.11)

| Modül          | Durum | Tamamlanan |
| -------------- | ----- | ---------- |
| Text Tools     | ✅    | 6/12       |
| PDF Tools      | 🚧    | 10/12      |
| Converters     | 🚧    | 0/12       |
| File & System  | 🚧    | 0/12       |
| Image Tools    | 🚧    | 0/12       |
| Network        | 🚧    | 0/12       |
| Quick Commands | 🚧    | 0/12       |
| Dev Tools      | 🚧    | 0/12       |
| TBD 9-12       | 🚧    | 0/48       |

## 🔮 Gelecek Özellikler

- [ ] Kalan 11 modülün implementasyonu
- [ ] Özelleştirilebilir klavye kısayolları
- [ ] Favoriler sistemi
- [ ] Geçmiş/son kullanılanlar
- [ ] Clipboard entegrasyonu
- [ ] Fuzzy search
- [ ] Command palette
- [ ] Makro kaydı

## 🤝 Katkıda Bulunma

Katkılarınızı bekliyoruz! Lütfen bir pull request göndermeden önce issue açın.

## 📄 Lisans

[Lisans bilgisi eklenecek]

---

**Not**: Bu proje aktif geliştirme aşamasındadır. Text Tools modülü hazır, diğer modüller planlanma/geliştirme aşamasındadır.
