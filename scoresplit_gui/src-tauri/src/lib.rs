extern crate scoresplit_core;

use std::{thread, time::Instant};

use scoresplit_core::StreamManager;
use tauri::{window, Emitter, Window};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
/// get frame from captures device(camera, video) using opencv
fn get_frame(window: Window) {
    if let Ok(mut stream_manager) = StreamManager::new() {
        thread::spawn(move || loop {
            let time = Instant::now();
            let value = stream_manager.get_frame_as_base64();
            // emit current frame
            window.emit("update_frame", value);
            println!("time : {:?}", time.elapsed());
        });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_frame])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
