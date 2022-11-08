#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::AppHandle;

fn build_window(handle: AppHandle, label: String, x: f64, y: f64) {
    tauri::window::WindowBuilder::new(&handle, label, tauri::WindowUrl::App("index.html".into()))
        .inner_size(300.0, 300.0)
        .position(500.0 - x, 400.0 - y)
        .build()
        .expect("Unable to create window with label {}");
}

#[tauri::command]
async fn command_create_window(handle: AppHandle, label: String, x: f64, y: f64) {
    tauri::window::WindowBuilder::new(&handle, label, tauri::WindowUrl::App("index.html".into()))
        .inner_size(300.0, 300.0)
        .position(500.0 - x, 400.0 - y)
        .build()
        .expect("Unable to create window in command");
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![command_create_window])
        .build(tauri::generate_context!())
        .expect("Error while running tauri application");

    build_window(app.handle(), "A".to_string(), -150.0, -150.0);
    build_window(app.handle(), "B".to_string(), 150.0, -150.0);

    app.run(|e, _| {});
}
