// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod textfunc;
mod pdffunc;
mod convertfunc;
mod utils;

#[cfg(target_os = "linux")]
use std::sync::{Arc, Mutex};
#[cfg(target_os = "linux")]
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
            .expect("Shortcur Register Failed")
            .with_handler(|app, shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    println!("Windows/Mac Kısayol: {:?}", shortcut);
                    utils::toggle_window(app);
                }
            })
            .build(),
    );

    builder
        .plugin(tauri_plugin_dialog::init())
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

            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
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
            textfunc::process_char_inspector,
            pdffunc::get_pdf_info,
            pdffunc::open_file,
            pdffunc::pdf_merge,
            pdffunc::pdf_split,
            pdffunc::images_to_pdf,
            pdffunc::pdf_to_images,
            pdffunc::pdf_compress,
            pdffunc::pdf_rotate,
            pdffunc::pdf_delete_pages,
            pdffunc::pdf_extract_text,
            pdffunc::pdf_remove_password,
            pdffunc::pdf_protect,
            pdffunc::pdf_watermark,
            pdffunc::pdf_read_metadata,
            pdffunc::pdf_metadata,
            convertfunc::convert_office,
            convertfunc::read_docx_binary,
            convertfunc::save_binary_file
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
