// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager, PhysicalPosition, PhysicalSize, WebviewWindow,
};

fn set_window_position(window: &WebviewWindow) -> Result<(), Box<dyn std::error::Error>> {
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
async fn create_new_window(app: tauri::AppHandle, id: String, title: String) -> Result<(), String> {
    if let Some(main_window) = app.get_webview_window("main") {
        main_window.hide().map_err(|e| e.to_string())?;
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

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let show_i = MenuItem::with_id(app, "show", "Göster", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Çıkış", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i,&quit_i])?;

            let _tray = TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = set_window_position(&window);
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app);

            let mainwindow = app.get_webview_window("main").unwrap();
            set_window_position(&mainwindow).ok();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_new_window])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                if let Err(e) = window.hide(){
                    eprintln!("Tray'e alınırken hata: {}", e);
                }
            }


            tauri::WindowEvent::Destroyed => {
                if window.label() != "main" {
                    if let Some(main_window) = window.get_webview_window("main") {
                        if let Err(e) = set_window_position(&main_window) {
                            eprintln!("Pencere ayarlanırken hata oluştu: {}", e);
                        }
                    }
                }
            }

            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("Error While Processing")
}
