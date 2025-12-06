// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, PhysicalPosition, WebviewWindow};

fn set_window_position(window: &WebviewWindow) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(monitor) = window.primary_monitor()? {
        let window_size = window.outer_size()?;
        let monitor_size = monitor.size();

        let x = (monitor_size.width as i32 - window_size.width as i32) / 2;
        let y = (monitor_size.height as f64 * 0.15) as i32;

        window.set_position(PhysicalPosition::new(x, y))?;
        window.show()?;
        window.set_focus()?;
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let mainwindow = app.get_webview_window("main").unwrap();
            set_window_position(&mainwindow);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error While Processing")
}
