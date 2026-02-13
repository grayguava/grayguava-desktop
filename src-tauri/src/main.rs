#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewUrl};
use std::net::ToSocketAddrs;

fn host_resolves(url: &str) -> bool {
    if let Ok(parsed) = url::Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            return (host, 443).to_socket_addrs().is_ok();
        }
    }
    true
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let handle = app.handle();

            window.on_navigation(move |url| {
                if !host_resolves(&url) {
                    let offline = tauri::path::resolve(
                        &handle.config(),
                        handle.package_info(),
                        "src/offline.html",
                        Some(tauri::path::BaseDirectory::Resource),
                    ).unwrap();

                    window.navigate(WebviewUrl::App(
                        offline.to_string_lossy().to_string()
                    ));

                    return false;
                }
                true
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
