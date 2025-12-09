// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager
};
use utils::set_window_position;
use utils::create_new_window;

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
