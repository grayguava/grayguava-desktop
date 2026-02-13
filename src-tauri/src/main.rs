#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewUrl, PageLoadEvent};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let handle = app.handle();

            window.on_page_load(move |event| {
                if let PageLoadEvent::Failed { .. } = event {
                    let offline = tauri::path::resolve(
                        &handle.config(),
                        handle.package_info(),
                        "src/offline.html",
                        Some(tauri::path::BaseDirectory::Resource),
                    ).unwrap();

                    window.load_url(WebviewUrl::App(
                        offline.to_string_lossy().to_string()
                    )).ok();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
