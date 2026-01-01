// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod textfunc;
mod utils;

use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use utils::create_new_window;
use utils::set_window_position;

#[cfg(not(target_os = "linux"))]
use tauri_plugin_global_shortcut::ShortcutState;

fn main() {
    let builder = tauri::Builder::default();

    // 2. WINDOWS & MACOS İÇİN PLUGIN
    #[cfg(not(target_os = "linux"))]
    let builder = builder.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcut("ctrl+g")
            .with_handler(|app, shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    println!("Windows/Mac Kısayol: {:?}", shortcut);
                    utils::toggle_window(app);
                }
            })
            .build(),
    );

    builder
        .setup(|app| {
            // ----------------------------------------------------------------
            // 3. LINUX İÇİN RAW INPUT LISTENER (RDEV)
            // ----------------------------------------------------------------
            #[cfg(target_os = "linux")]
            {
                std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

                let app_handle = app.handle().clone();

                thread::spawn(move || {
                    use rdev::{listen, EventType, Key};

                    let ctrl_pressed = Arc::new(Mutex::new(false));
                    let ctrl_clone = ctrl_pressed.clone();

                    println!("Linux Klavye Dinleyicisi Başlatıldı (Ctrl+G bekleniyor...)");

                    if let Err(error) = listen(move |event| {
                        match event.event_type {
                            EventType::KeyPress(Key::ControlLeft)
                            | EventType::KeyPress(Key::ControlRight) => {
                                *ctrl_clone.lock().unwrap() = true;
                            }
                            EventType::KeyRelease(Key::ControlLeft)
                            | EventType::KeyRelease(Key::ControlRight) => {
                                *ctrl_clone.lock().unwrap() = false;
                            }
                            EventType::KeyPress(Key::KeyG) => {
                                if *ctrl_clone.lock().unwrap() {
                                    println!("Linux Raw Input: Ctrl+G Yakalandı!");

                                    // --- DÜZELTİLEN KISIM BURASI ---
                                    // Closure içine göndermek için YENİ bir kopya oluşturuyoruz.
                                    // app_handle: Fonksiyonu çağırmak (executor) için kullanılıyor.
                                    // handle_for_closure: İçeriye taşınmak (moved) için kullanılıyor.
                                    let handle_for_closure = app_handle.clone();

                                    let _ = app_handle.run_on_main_thread(move || {
                                        utils::toggle_window(&handle_for_closure);
                                    });
                                }
                            }
                            _ => {}
                        }
                    }) {
                        eprintln!("Klavye dinleme hatası: {:?}", error);
                    }
                });
            }

            let show_i = MenuItem::with_id(app, "show", "Goster", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Cikis", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => app.exit(0),
                    "show" => utils::toggle_window(app),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } => {
                        utils::toggle_window(tray.app_handle());
                    }
                    _ => {}
                })
                .build(app);

            let mainwindow = app.get_webview_window("main").unwrap();
            set_window_position(&mainwindow, None, None).ok();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_new_window,
            textfunc::process_text_diff,
            textfunc::process_text_reg,
            textfunc::process_strip,
            textfunc::process_extractor,
            textfunc::process_string_escape,
            textfunc::process_slug_gen,
            textfunc::process_jwt_decode,
            textfunc::process_markdown_preview,
            textfunc::process_lorem,
            textfunc::process_obfuscator,
            textfunc::process_char_inspector
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if window.label() == "main" {
                    api.prevent_close();
                    let _ = window.hide();
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
                        set_window_position(&parent, None, None).map_err(|e| e.to_string())?;
                        parent.set_focus().map_err(|e| e.to_string())?;
                        Ok(())
                    })();
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("Error While Processing")
}
