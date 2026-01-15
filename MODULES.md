# SimpleTools - Modül Planlama

Bu dosya, SimpleTools projesindeki 12 modül ve her modüldeki 12 fonksiyonun detaylı planlamasını içerir.

## Modül 1: Text Tools ✅ (TAMAMLANDI)

**Kısayol**: T  
**Durum**: Hazır  
**Backend**: `src-tauri/src/textfunc.rs`

### Fonksiyonlar (12/12)

1. **Regex Tester** (R) ✅

   - Regex pattern test etme
   - Match highlighting
   - Capture groups gösterimi

2. **Text Diff** (D) ✅

   - İki metin karşılaştırma
   - Side-by-side görünüm
   - Fark vurgulama

3. **String Tools** (S) ✅

   - Case dönüşümleri (upper, lower, title, camel, snake, kebab)
   - String manipülasyonları

4. **JWT Decoder** (J) ✅

   - JWT token decode
   - Header, payload, signature gösterimi
   - Validation

5. **Slug Generator** (L) ✅

   - URL-friendly slug oluşturma
   - Özel karakter temizleme

6. **Text Strip** (T) ✅

   - Whitespace temizleme
   - Satır sonu normalizasyonu

7. **Lorem Ipsum Generator** (I) - EKLENECEK

   - Placeholder metin oluşturma
   - Paragraf/kelime sayısı ayarı

8. **Base64 Encoder/Decoder** (B) - EKLENECEK

   - Base64 encode/decode
   - Dosya desteği

9. **URL Encoder/Decoder** (U) - EKLENECEK

   - URL encode/decode
   - Query string parser

10. **Markdown Preview** (M) - EKLENECEK

    - Markdown to HTML
    - Live preview

11. **Character Counter** (C) - EKLENECEK

    - Karakter, kelime, satır sayısı
    - Reading time hesaplama

12. **Text Sorter** (O) - EKLENECEK
    - Alfabetik sıralama
    - Reverse, unique, shuffle

---

## Modül 2: PDF Tools 🚧

**Kısayol**: P  
**Durum**: Geliştiriliyor  
**Backend**: `src-tauri/src/pdffunc.rs`

### Fonksiyonlar (1/12)

1. **PDF Merger (Birleştirici)** (M) ✅ **Gelişmiş Özelliklerle**

   - Dağınık PDF dosyalarını sıraya dizip tek dosya yapar
   - **Akıllı Varsayılanlar**: Otomatik çıktı yolu ve dosya adı oluşturma
   - **Dosya Metadata**: Sayfa sayısı, dosya boyutu gösterimi
   - **Sürükle-Bırak Sıralama**: Dosyaları sürükleyerek sıralama
   - **Şifreli Dosya Tespiti**: 🔒 ikonu ile uyarı
   - **Başarı Geri Bildirimi**: "Klasörü Aç" ve "Dosyayı Aç" butonları
   - Neden: Müşteriye 5 ayrı ek göndermek profesyonel durmaz, tek dosya istenir

2. **PDF Splitter (Ayırıcı)** (S)

   - 100 sayfalık bir raporun sadece 5-10 arasındaki sayfalarını alıp yeni bir PDF yapar
   - "Her sayfayı ayrı dosya yap" seçeneği
   - Neden: "Bana sadece ilgili sayfayı gönder" dendiğinde koca dosyayı atmamak için

3. **Images to PDF (Resimden PDF'e)** (I)

   - Telefondan çekilmiş veya taranmış JPG/PNG evrak fotoğraflarını seçip tek bir PDF haline getirir
   - Neden: Vize başvurusu veya IK evrakları genelde fotoğraf değil, tek PDF olarak istenir

4. **PDF to Images (PDF'ten Resme)** (G)

   - PDF'in her sayfasını yüksek kaliteli JPG veya PNG olarak dışarı aktarır
   - Neden: Bir sunumun tek bir sayfasını alıp Instagram'a veya WhatsApp'a resim olarak atmak için

5. **Compress PDF (Sıkıştırıcı)** (C)

   - 20 MB'lık dosyayı kaliteyi çok bozmadan 2-3 MB'a düşürür
   - Neden: E-posta ekleri genelde 25MB sınırına takılır veya upload siteleri dosya boyutu sınırı koyar

6. **Rotate Pages (Sayfa Döndür)** (R)

   - Yanlış taranmış sayfaları 90/180 derece döndürüp kaydeder
   - Neden: Tarayıcıdan ters çıkmış bir belgeyi okumak için boyun fıtığı olmamak gerekir

7. **Delete Pages (Sayfa Sil)** (D)

   - PDF içinden seçilen sayfaları uçurur
   - Neden: Yeniden tarama yapmadan aradaki çürük elmaları ayıklamak için

8. **Extract Text (Metin Kazıyıcı)** (T)

   - PDF'in içindeki seçilebilir yazıları saf txt olarak dışarı alır (OCR değil, gömülü metin)
   - Neden: Kopyala-yapıştır yapmanın yasak olduğu veya zor olduğu durumlarda içeriği almak için

9. **Remove Password (Şifre/Kilit Kaldır)** (U)

   - Şifresini bildiğin dosyanın şifresini kalıcı olarak siler
   - Owner Password removal
   - Neden: Kendi kredi kartı ekstreni arşivlerken her seferinde şifre girmek istemezsin

10. **Protect PDF (Şifrele)** (P)

    - Dosyaya açılış şifresi (User Password) veya yazdırma yasağı (Owner Password) koyar
    - Neden: Maaş bordrosu veya gizli proje dosyası gönderirken güvenlik şarttır

11. **Watermark (Filigran Ekle)** (W)

    - Sayfaların üzerine çapraz şekilde yarı saydam filigran yazar
    - Neden: Belgenin statüsünü belirtmek veya izinsiz kullanımını engellemek için

12. **Metadata Editor (Künye Düzenleyici)** (E)
    - Dosyanın Yazar, Başlık gibi arka plan bilgilerini değiştirir
    - Neden: Başkasından aldığın bir ödevi teslim etmeden önce "Author" kısmında başkasının adının yazmasını istemezsin

### Gelişmiş Özellikler (PDF Merger)

- ✅ **Smart Defaults**: Otomatik çıktı yolu ve dosya adı oluşturma
- ✅ **File Metadata**: Sayfa sayısı, dosya boyutu, şifreleme durumu
- ✅ **Drag & Drop Sorting**: Dosyaları sürükleyerek sıralama
- ✅ **Encrypted Detection**: Şifreli dosya tespiti ve uyarı
- ✅ **Success Feedback**: Klasör/dosya açma butonları
- ✅ **Error Handling**: Detaylı hata mesajları

---

## Modül 3: Converters 🚧

**Kısayol**: C  
**Durum**: Planlanıyor  
**Backend**: `src-tauri/src/convertfunc.rs` (oluşturulacak)

### Fonksiyonlar (0/12)

1. **JSON to YAML** (J)

   - JSON ↔ YAML dönüşümü
   - Syntax highlighting

2. **CSV to JSON** (C)

   - CSV ↔ JSON dönüşümü
   - Delimiter seçimi

3. **XML to JSON** (X)

   - XML ↔ JSON dönüşümü
   - Pretty print

4. **Color Converter** (O)

   - HEX, RGB, HSL dönüşümleri
   - Color picker

5. **Unit Converter** (U)

   - Uzunluk, ağırlık, sıcaklık
   - Çoklu birim desteği

6. **Timestamp Converter** (T)

   - Unix timestamp ↔ Date
   - Timezone desteği

7. **Number Base Converter** (N)

   - Binary, Octal, Decimal, Hex
   - Çoklu base dönüşümü

8. **Image Format Converter** (I)

   - PNG, JPG, WebP, SVG
   - Resize seçeneği

9. **Audio Converter** (A)

   - MP3, WAV, OGG, FLAC
   - Bitrate ayarı

10. **Video Converter** (V)

    - MP4, WebM, AVI, MKV
    - Codec seçimi

11. **Font Converter** (F)

    - TTF, OTF, WOFF, WOFF2
    - Subset oluşturma

12. **Markdown to HTML** (M)
    - Markdown → HTML
    - Template desteği

---

## Modül 4: File & System 🚧

**Kısayol**: F  
**Durum**: Planlanıyor  
**Backend**: `src-tauri/src/filefunc.rs` (oluşturulacak)

### Fonksiyonlar (0/12)

1. **Hash Generator** (H)

   - MD5, SHA1, SHA256, SHA512
   - Dosya hash'leme

2. **File Renamer** (R)

   - Toplu dosya yeniden adlandırma
   - Regex pattern desteği

3. **Duplicate Finder** (D)

   - Duplicate dosya bulma
   - Hash-based karşılaştırma

4. **Disk Usage Analyzer** (U)

   - Klasör boyutu analizi
   - Görsel grafik

5. **File Splitter** (S)

   - Büyük dosyaları parçalama
   - Birleştirme

6. **Checksum Verifier** (C)

   - Checksum doğrulama
   - Çoklu algoritma

7. **File Permissions** (P)

   - Dosya izinleri görüntüleme
   - Chmod calculator

8. **Directory Tree** (T)

   - Klasör yapısı görselleştirme
   - Export to text

9. **File Watcher** (W)

   - Dosya değişikliklerini izleme
   - Real-time monitoring

10. **Temp File Cleaner** (L)

    - Geçici dosya temizleme
    - Safe delete

11. **File Metadata Viewer** (M)

    - EXIF, ID3 tags
    - Metadata düzenleme

12. **Symbolic Link Manager** (Y)
    - Symlink oluşturma
    - Link yönetimi

---

## Modül 5: Image Tools 🚧

**Kısayol**: I  
**Durum**: Planlanıyor  
**Backend**: `src-tauri/src/imagefunc.rs` (oluşturulacak)

### Fonksiyonlar (0/12)

1. **Image Resizer** (R)

   - Boyutlandırma
   - Aspect ratio koruma

2. **Image Compressor** (C)

   - Lossy/lossless sıkıştırma
   - Kalite ayarı

3. **Image Cropper** (O)

   - Kırpma aracı
   - Preset boyutlar

4. **Image Filters** (F)

   - Grayscale, blur, sharpen
   - Çoklu filter

5. **Watermark Tool** (W)

   - Resim/metin filigran
   - Pozisyon ayarı

6. **Background Remover** (B)

   - Arka plan silme
   - AI-powered

7. **Image to Base64** (A)

   - Base64 encode
   - Data URI oluşturma

8. **QR Code Generator** (Q)

   - QR kod oluşturma
   - Özelleştirme

9. **Barcode Generator** (D)

   - Barkod oluşturma
   - Çoklu format

10. **Image Metadata Editor** (M)

    - EXIF düzenleme
    - Metadata temizleme

11. **Image Collage** (L)

    - Kolaj oluşturma
    - Layout seçenekleri

12. **Screenshot Tool** (S)
    - Ekran görüntüsü
    - Annotation

---

## Modül 6: Network 🚧

**Kısayol**: N  
**Durum**: Planlanıyor  
**Backend**: `src-tauri/src/networkfunc.rs` (oluşturulacak)

### Fonksiyonlar (0/12)

1. **IP Info** (I)

   - IP bilgisi görüntüleme
   - Geolocation

2. **Port Scanner** (P)

   - Port tarama
   - Service detection

3. **DNS Lookup** (D)

   - DNS sorguları
   - A, MX, TXT records

4. **Ping Tool** (G)

   - Ping testi
   - Latency ölçümü

5. **Traceroute** (T)

   - Route izleme
   - Hop analizi

6. **WHOIS Lookup** (W)

   - Domain bilgisi
   - Registrar info

7. **SSL Certificate Checker** (S)

   - SSL sertifika kontrolü
   - Expiry date

8. **HTTP Headers** (H)

   - HTTP header görüntüleme
   - Request/response

9. **URL Shortener** (U)

   - URL kısaltma
   - Custom alias

10. **Network Speed Test** (E)

    - Download/upload hızı
    - Latency testi

11. **MAC Address Lookup** (M)

    - MAC vendor lookup
    - OUI database

12. **Subnet Calculator** (C)
    - CIDR hesaplama
    - IP range

---

## Modül 7: Quick Commands 🚧

**Kısayol**: Q  
**Durum**: Planlanıyor  
**Backend**: `src-tauri/src/quickfunc.rs` (oluşturulacak)

### Fonksiyonlar (0/12)

1. **UUID Generator** (U)

   - UUID v4 oluşturma
   - Toplu üretim

2. **Password Generator** (P)

   - Güçlü şifre oluşturma
   - Özelleştirilebilir

3. **Random Number** (N)

   - Random sayı üretimi
   - Range seçimi

4. **Lorem Ipsum** (L)

   - Placeholder metin
   - Paragraf/kelime

5. **Cron Expression** (C)

   - Cron ifadesi oluşturma
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
    - Emoji arama
    - Copy to clipboard

---

## Modül 8: Dev Tools 🚧

**Kısayol**: D  
**Durum**: Planlanıyor  
**Backend**: `src-tauri/src/devfunc.rs` (oluşturulacak)

### Fonksiyonlar (0/12)

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

   - Diff görüntüleme
   - Syntax highlight

8. **API Tester** (A)

   - REST API test
   - Request builder

9. **GraphQL Playground** (Q)

   - GraphQL query test
   - Schema explorer

10. **WebSocket Tester** (W)

    - WebSocket bağlantı
    - Message testing

11. **Code Snippet Manager** (S)

    - Snippet saklama
    - Syntax highlight

12. **Regex Builder** (R)
    - Visual regex builder
    - Pattern library

---

## Modül 9-12: TBD 🚧

Gelecekte eklenecek modüller için fikirler:

### Potansiyel Modüller

- **Security Tools**: Encryption, hashing, password audit
- **Database Tools**: Query builder, schema designer
- **Git Tools**: Commit helper, branch manager
- **Math Tools**: Calculator, equation solver
- **Data Tools**: Data generator, faker
- **Crypto Tools**: Cryptocurrency converter, wallet
- **Time Tools**: Timer, stopwatch, pomodoro
- **Note Tools**: Quick notes, markdown editor

---

## İlerleme Özeti

| Modül          | Durum | Tamamlanan | Kalan   |
| -------------- | ----- | ---------- | ------- |
| Text Tools     | ✅    | 6/12       | 6       |
| PDF Tools      | 🚧    | 0/12       | 12      |
| Converters     | 🚧    | 0/12       | 12      |
| File & System  | 🚧    | 0/12       | 12      |
| Image Tools    | 🚧    | 0/12       | 12      |
| Network        | 🚧    | 0/12       | 12      |
| Quick Commands | 🚧    | 0/12       | 12      |
| Dev Tools      | 🚧    | 0/12       | 12      |
| TBD 9          | 🚧    | 0/12       | 12      |
| TBD 10         | 🚧    | 0/12       | 12      |
| TBD 11         | 🚧    | 0/12       | 12      |
| TBD 12         | 🚧    | 0/12       | 12      |
| **TOPLAM**     |       | **6/144**  | **138** |

**İlerleme**: %4.17
