use std::sync::Mutex;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewWindow};

static LAST_ACTIVE_WINDOW: Mutex<Option<String>> = Mutex::new(None);

use mouse_position::mouse_position::Mouse;

pub fn set_window_position(window: &WebviewWindow, width: Option<u32>, height: Option<u32>) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Layer Tespiti
    // Label veya Path üzerinden katman tespiti.
    // Main -> Layer 1
    // /text -> Layer 2 (1 slash)
    // /text/diff -> Layer 3 (2 slash)
    
    // Basitçe labela bakalım:
    let label = window.label();
    
    // NOT: window.label() "text/diff" şeklinde geliyor oluştururken verdiğimiz id'ye göre.
    // create_new_window'da id'yi label olarak kullanıyoruz.
    // "text" -> Layer 2
    // "text/diff" -> Layer 3
    let slash_count = label.matches('/').count();
    
    let layer = if label == "main" { 
        1 
    } else if slash_count == 0 { 
        2 // "text"
    } else { 
        3 // "text/diff"
    };

    println!("LAYER DEBUG: Window '{}' detected as Layer {}", label, layer);

    // 2. Mouse ve Monitör Tespiti (Global)
    let mut mouse_x = 0;
    let mut mouse_y = 0;
    
    match Mouse::get_mouse_position() {
        Mouse::Position { x, y } => {
            mouse_x = x;
            mouse_y = y;
        },
        Mouse::Error => eprintln!("Error getting mouse position"),
    }

    let monitors = window.available_monitors()?;
    let mut target_monitor = window.current_monitor()?.unwrap_or(
        window.primary_monitor()?.unwrap()
    );

    for m in &monitors {
        let pos = m.position();
        let size = m.size();
        
        // Debug
        println!("  - Monitor: Pos({},{}), Size({}x{})", pos.x, pos.y, size.width, size.height);

        let m_min_x = pos.x;
        let m_max_x = pos.x + size.width as i32;
        let m_min_y = pos.y;
        let m_max_y = pos.y + size.height as i32;

        if mouse_x >= m_min_x && mouse_x < m_max_x && mouse_y >= m_min_y && mouse_y < m_max_y {
            println!("  -> HIT! Mouse is on this monitor.");
            target_monitor = m.clone();
            // break; // Break'i kaldırdım, tüm monitörleri görelim. Son bulunan (en üst katman?) geçerli olsun.
        }
    }

    // 3. Boyut ve Pozisyon Hesaplama
    let m_pos = target_monitor.position();
    let m_size = target_monitor.size();

    let mut final_width = 0;
    let mut final_height = 0;
    let mut final_x = 0;
    let mut final_y = 0;

    if layer == 1 || layer == 2 {
        // Katman 1 ve 2: %50 Genişlik, %45 Yükseklik, Tam Orta
        final_width = (m_size.width as f64 * 0.60) as u32;
        final_height = (m_size.height as f64 * 0.65) as u32;
        
        final_x = m_pos.x + (m_size.width as i32 - final_width as i32) / 2;
        final_y = m_pos.y + (m_size.height as i32 - final_height as i32) / 2;
    } else {
        // Katman 3: Varsayılan (veya mevcut), Ortanın biraz üstü
        // Önce mevcut boyuta bakalım, eğer 0 ise (yeni pencere) varsayılanı alalım
        let current_size = window.inner_size()?;
        // Eğer create_new_window'dan width/height geldiyse onu kullan, yoksa mevcudu, o da yoksa 1000x800
        // (Burada parametre olarak gelen width/height'i önceliklendiriyoruz)
        final_width = width.unwrap_or(if current_size.width > 0 { current_size.width } else { 1000 });
        final_height = height.unwrap_or(if current_size.height > 0 { current_size.height } else { 800 });

        final_x = m_pos.x + (m_size.width as i32 - final_width as i32) / 2;
        // Y: Ortanın üstü (%35)
        final_y = m_pos.y + ((m_size.height as f64 - final_height as f64) * 0.35) as i32;
    }

    println!("POS DEBUG: Layer {} -> {}x{} at {},{}", layer, final_width, final_height, final_x, final_y);

    window.set_size(PhysicalSize::new(final_width, final_height))?;
    window.set_position(PhysicalPosition::new(final_x, final_y))?;
    
    window.show()?;
    window.set_focus()?;

    Ok(())
}

#[tauri::command]
pub async fn create_new_window(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    id: String,
    title: String,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<(), String> {
    if let Err(e) = window.hide() {
        eprintln!("parent window hidelanirken bir sorun olustu {}", e)
    }

    if let Some(exis_window) = app.get_webview_window(&id) {
        if let Err(e) = set_window_position(&exis_window, width, height) {
            eprintln!("Var olan window gosterilirken hata olustu {}", e);
        }
        return Ok(());
    }

    let name = id.clone();
    let path = String::from("/") + &(id.clone());
    println!("Creating new window: {} at path: {}", name, path);

    let _new_window =
        tauri::WebviewWindowBuilder::new(&app, id, tauri::WebviewUrl::App(path.into()))
            .title(&title)
            .always_on_top(true)
            .resizable(false)
            .visible(false)
            .transparent(true)
            .decorations(false)
            .build()
            .map_err(|e| e.to_string())?;

    let current_window = app.get_webview_window(&name).unwrap();
    if let Err(e) = set_window_position(&current_window, width, height) {
        eprintln!("Yeni pencerenin pozisyonu ayarlanamadi {}", e);
    }

    Ok(())
}

pub fn toggle_window(app: &AppHandle) {
    let windows = app.webview_windows();

    // -----------------------------------------------------------
    // SENARYO 1: EKRANDA GÖRÜNEN BİR PENCERE VAR MI?
    // -----------------------------------------------------------
    for (label, window) in &windows {
        if window.is_visible().unwrap_or(false) {
            // Evet, görünür bir pencere bulduk (Örn: regex veya text)

            // 1. Bu pencerenin adını hafızaya kaydet
            if let Ok(mut last) = LAST_ACTIVE_WINDOW.lock() {
                *last = Some(label.clone());
                println!("Hafızaya alındı: {}", label); // Debug
            }

            // 2. Pencereyi gizle
            let _ = window.hide();

            // 3. İşlem tamam, çık (Diğer pencerelere bakma)
            return;
        }
    }

    // -----------------------------------------------------------
    // SENARYO 2: HİÇBİR PENCERE GÖRÜNMÜYOR
    // -----------------------------------------------------------

    // 1. Önce hafızaya bakalım: En son kimi kapattık?
    let target_label = if let Ok(last) = LAST_ACTIVE_WINDOW.lock() {
        last.clone() // İsmi kopyala (örn: "regex")
    } else {
        None
    };

    // 2. Hafızadaki pencereyi bulup açmaya çalışalım
    if let Some(label) = target_label {
        if let Some(window) = app.get_webview_window(&label) {
            println!("Hafızadan geri çağrılıyor: {}", label);

            // Konumlandır ve Göster
            // Not: Hafızadan çağırırken, en son hangi boyutta olduğu veya default boyutu neydi bilmiyoruz.
            // Bu basit implementasyonda None, None göndererek 800x800'e dönebilir veya mevcut boyutu koruyabiliriz.
            // set_window_position parametreleri Option olduğu için None gönderirsek 800x800 resetler.
            // Eğer pencerenin kendi boyutunu korumasını istiyorsak set_window_position logic'ini biraz daha akıllı yapmalıyız
            // ya da burada sadece "show" demeliyiz.
            // Ancak "utils::toggle_window" logic'i genellikle "main" context'inden çağrılıyor.
            
            // Kullanıcı UX olarak önceki boyutta kalmasını ister.
            // set_window_position'ı modifiye edip eğer width/height None ise resize yapmamasını sağlayabiliriz.
            // Fakat yukarıdaki implementasyonda unwrap_or(800) yaptık.
            
            // İyileştirme: Window zaten varsa resize etme?
            let _ = set_window_position(&window, None, None);
            if window.is_minimized().unwrap_or(false) {
                let _ = window.unminimize();
            }
            let _ = window.show();
            let _ = window.set_focus();
            return; // Başarıyla açtık, çık.
        }
    }

    // 3. Hafıza boşsa veya hafızadaki pencere artık yoksa (kapatılmışsa)
    // Varsayılan olarak "main" penceresini aç.
    if let Some(main_win) = app.get_webview_window("main") {
        println!("Varsayılan (Main) açılıyor");
        let _ = set_window_position(&main_win, None, None);
        if main_win.is_minimized().unwrap_or(false) {
            let _ = main_win.unminimize();
        }
        let _ = main_win.show();
        let _ = main_win.set_focus();
    }
}
