use tauri::{Manager, PhysicalPosition, PhysicalSize, WebviewWindow};

pub fn set_window_position(window: &WebviewWindow) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(monitor) = window.current_monitor()? {
        let monitor_size = monitor.size();
        let target_width = 800;
        let target_height = 800;
        let x = (monitor_size.width as i32 - target_width) / 2;
        let y = (monitor_size.height as f64 * 0.10) as i32;
        window.set_size(PhysicalSize::new(target_width as u32, target_height as u32))?;
        window.set_position(PhysicalPosition::new(x, y))?;
    }
    window.show()?;
    window.set_focus()?;

    Ok(())
}

#[tauri::command]
pub async fn create_new_window(app: tauri::AppHandle, id: String, title: String) -> Result<(), String> {
    if let Some(main_window) = app.get_webview_window("main") {
        main_window.hide().map_err(|e| e.to_string())?;
    }

    if let Some(exis_window) = app.get_webview_window(&id) {
            if let Err(e) = set_window_position(&exis_window) {
                eprintln!("Var olan window gosterilirken hata olustu {}", e);
            }
            return Ok(());
    }

    let name = id.clone();
    let path = String::from("/") + &(id.clone());

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
    if let Err(e) = set_window_position(&current_window) {
        eprintln!("Yeni pencerenin pozisyonu ayarlanamadi {}", e);
    }

    Ok(())
}

