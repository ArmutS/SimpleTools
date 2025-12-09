// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, PhysicalPosition, PhysicalSize, WebviewWindow};

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

    let _new_window =
        tauri::WebviewWindowBuilder::new(&app, id, tauri::WebviewUrl::App("/text".into()))
            .title(&title)
            .always_on_top(true)
            .resizable(false)
            .visible(false)
            .transparent(true)
            .decorations(false)
            .build()
            .map_err(|e| e.to_string())?;

    let a = app.get_webview_window(&name).unwrap();
    set_window_position(&a).ok();

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let mainwindow = app.get_webview_window("main").unwrap();
            set_window_position(&mainwindow).ok();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_new_window])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Destroyed => {
                if window.label() != "main" {
                    if let Some(main_window) = window.get_webview_window("main") {
                       main_window.show();
                    }
                    
                }
            }

            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("Error While Processing")
}
