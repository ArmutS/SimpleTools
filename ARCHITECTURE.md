# SimpleTools - Mimari Dokümantasyon

## Proje Genel Bakış

SimpleTools, vim-benzeri klavye kısayolları ile hızlı erişim sağlayan modüler bir araç koleksiyonudur. Alfred benzeri bir arayüz ile Alt+Space tuşu ile açılır ve her modüle özel tuş kombinasyonları ile hızlı navigasyon sağlar.

## Teknoloji Stack

- **Frontend**: Svelte + TypeScript
- **Backend**: Rust (Tauri)
- **UI Framework**: Tauri v2
- **Build Tool**: Vite

## Proje Yapısı

```
SimpleTools/
├── src/                          # Frontend (Svelte)
│   ├── routes/                   # SvelteKit routes
│   │   ├── +page.svelte         # Ana sayfa (modül seçici)
│   │   ├── text/                # Text Tools modülü
│   │   ├── pdf/                 # PDF Tools modülü (yapılacak)
│   │   ├── convert/             # Converters modülü (yapılacak)
│   │   ├── file/                # File & System modülü (yapılacak)
│   │   ├── image/               # Image Tools modülü (yapılacak)
│   │   ├── network/             # Network modülü (yapılacak)
│   │   ├── quickcmd/            # Quick Commands modülü (yapılacak)
│   │   └── dev/                 # Dev Tools modülü (yapılacak)
│   ├── themes/                  # Tema dosyaları
│   └── app.css                  # Global stiller
├── src-tauri/                   # Backend (Rust)
│   ├── src/
│   │   ├── main.rs             # Ana Tauri uygulaması
│   │   ├── utils.rs            # Pencere yönetimi ve pozisyonlama
│   │   └── textfunc.rs         # Text Tools fonksiyonları
│   └── Cargo.toml
└── static/                      # Statik dosyalar
```

## Modül Sistemi

### Mevcut Modüller (12 Toplam)

| #   | Modül ID | İsim          | Kısayol | Durum        | Fonksiyon Sayısı |
| --- | -------- | ------------- | ------- | ------------ | ---------------- |
| 1   | text     | Text Tools    | T       | ✅ Hazır     | 12               |
| 2   | pdf      | PDF Tools     | P       | 🚧 %83       | 12               |
| 3   | convert  | Converters    | C       | 🚧 Yapılacak | 12               |
| 4   | file     | File & System | F       | 🚧 Yapılacak | 12               |
| 5   | image    | Image Tools   | I       | 🚧 Yapılacak | 12               |
| 6   | network  | Network       | N       | 🚧 Yapılacak | 12               |
| 7   | quickcmd | Quick Cmds    | Q       | 🚧 Yapılacak | 12               |
| 8   | dev      | Dev Tools     | D       | 🚧 Yapılacak | 12               |
| 9   | TBD      | Coming Soon   | -       | 🚧 Yapılacak | 12               |
| 10  | TBD      | Coming Soon   | -       | 🚧 Yapılacak | 12               |
| 11  | TBD      | Coming Soon   | -       | 🚧 Yapılacak | 12               |
| 12  | TBD      | Coming Soon   | -       | 🚧 Yapılacak | 12               |

**Toplam**: 144 fonksiyon (12 modül × 12 fonksiyon)

## Klavye Kısayolları Sistemi

### Global Kısayollar

- **Alt+Space**: Uygulamayı aç/kapat (toggle)
- **Ctrl+G**: Linux klavye dinleyicisi (global shortcut)
- **Esc**: Mevcut pencereyi kapat

### Modül Navigasyonu

Ana ekrandan modüllere erişim:

- **T**: Text Tools
- **P**: PDF Tools
- **C**: Converters
- **F**: File & System
- **I**: Image Tools
- **N**: Network
- **Q**: Quick Commands
- **D**: Dev Tools

### Fonksiyon Navigasyonu (Örnek: Text Tools)

Text Tools içinde:

- **R**: Regex Tester
- **D**: Text Diff
- **S**: String Tools
- **J**: JWT Decoder
- **L**: Slug Generator
- **T**: Text Strip
- _(6 fonksiyon daha eklenecek)_

## Pencere Katman Sistemi

Uygulama 3 katmanlı bir pencere sistemi kullanır:

### Layer 1: Ana Pencere (main)

- Modül seçici ekranı
- Boyut: Ekranın %60 genişlik, %65 yükseklik
- Pozisyon: Tam ortada

### Layer 2: Modül Pencereleri (text, pdf, vb.)

- Modül fonksiyon listesi
- Boyut: Ekranın %60 genişlik, %65 yükseklik
- Pozisyon: Tam ortada

### Layer 3: Fonksiyon Pencereleri (text/diff, text/regex, vb.)

- Gerçek araç arayüzü
- Boyut: Fonksiyona özel (varsayılan 1000×800)
- Pozisyon: Ortanın %35 üstü

## Özellikler

### ✅ Tamamlanan Özellikler

- [x] Global klavye kısayolu sistemi (Alt+Space)
- [x] Çoklu monitör desteği
- [x] Mouse pozisyonuna göre pencere yerleştirme
- [x] Katmanlı pencere yönetimi
- [x] Text Tools modülü (12 fonksiyon)
- [x] PDF Tools modülü (10/12 fonksiyon)
- [x] Tema sistemi (5 tema)
- [x] Vim-like navigasyon

### 🚧 Yapılacaklar

- [ ] Kalan 10 modülün implementasyonu
- [ ] Her modül için 12 fonksiyon
- [ ] Ayarlar sistemi
- [ ] Klavye kısayolu özelleştirme
- [ ] Geçmiş/favoriler sistemi
- [ ] Clipboard entegrasyonu

## Geliştirme Notları

### Yeni Modül Ekleme

1. `src/routes/+page.svelte` içinde `tools` dizisine ekle
2. `src/routes/{module-id}/` klasörü oluştur
3. `src-tauri/src/{module}func.rs` dosyası oluştur
4. Rust fonksiyonlarını `main.rs`'e kaydet

### Yeni Fonksiyon Ekleme

1. Modül klasöründe `{function-name}/+page.svelte` oluştur
2. Rust backend fonksiyonunu ilgili `{module}func.rs`'e ekle
3. `#[tauri::command]` ile işaretle
4. `main.rs`'de `.invoke_handler()` içine ekle

## Performans Optimizasyonları

- Pencere pozisyonları cache'leniyor
- Son aktif pencere hafızada tutuluyor
- Lazy loading ile modüller ihtiyaç anında yükleniyor
- Rust backend ile hızlı işlem

## Güvenlik

- Tauri'nin built-in güvenlik özellikleri
- CSP (Content Security Policy) aktif
- IPC güvenli invoke sistemi
- Dosya sistemi erişimi sınırlı

## Lisans

[Lisans bilgisi eklenecek]
