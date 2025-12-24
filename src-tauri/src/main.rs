// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod textfunc;
mod utils;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use utils::create_new_window;
use utils::set_window_position;

fn main() {
    //Linux WEBKIT_FIX
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    tauri::Builder::default()
        .setup(|app| {
            let show_i = MenuItem::with_id(app, "show", "Goster", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Cikis", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

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
        .invoke_handler(tauri::generate_handler![
            create_new_window,
            textfunc::process_text_diff,
            textfunc::process_text_reg,
            textfunc::process_strip,
            textfunc::process_extractor
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if window.label() == "main" {
                    api.prevent_close();
                    if let Err(e) = window.hide() {
                        eprintln!("Tray'e alinirken hata: {}", e);
                    }
                }
            }

            tauri::WindowEvent::Destroyed => {
                let app_handle = window.app_handle();
                let parent_label = if let Some((prefix, _suffix)) = window.label().rsplit_once("/")
                {
                    prefix
                } else {
                    "main"
                };

                if let Some(parent) = app_handle.get_webview_window(parent_label) {
                    let _ = (|| -> Result<(), String> {
                        set_window_position(&parent).map_err(|e| e.to_string())?;
                        parent.set_focus().map_err(|e| e.to_string())?;
                        Ok(())
                    })()
                    .map_err(|e| eprintln!("Ana pencere islem hatasi{}", e));
                }
            }

            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("Error While Processing")
}
