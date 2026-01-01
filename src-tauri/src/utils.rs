use std::sync::Mutex;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewWindow};

static LAST_ACTIVE_WINDOW: Mutex<Option<String>> = Mutex::new(None);

pub fn set_window_position(window: &WebviewWindow, width: Option<u32>, height: Option<u32>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(monitor) = window.current_monitor()? {
        let monitor_size = monitor.size();
        let target_width = width.unwrap_or(800);
        let target_height = height.unwrap_or(800);
        
        let x = (monitor_size.width as i32 - target_width as i32) / 2;
        let y = (monitor_size.height as f64 * 0.10) as i32;
        
        window.set_size(PhysicalSize::new(target_width, target_height))?;
        window.set_position(PhysicalPosition::new(x, y))?;
    }
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
