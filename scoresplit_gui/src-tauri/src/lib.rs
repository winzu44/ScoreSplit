extern crate scoresplit_core;

use scoresplit_core::stream_manager::StreamManager;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};
use tauri::{window, Emitter, Listener, Window};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
/// get frame from captures device(camera) using opencv
fn get_frame(window: Window) {
    let is_streaming = Arc::new(Mutex::new(true));
    // add event listner for stop streaming
    let cloned_is_streaming = Arc::clone(&is_streaming);
    window.listen("stop_stream", move |_| {
        if let Ok(mut is_streaming) = cloned_is_streaming.lock() {
            *is_streaming = false;
        }
    });

    if let Ok(mut stream_manager) = StreamManager::new() {
        let cloned_is_streaming = Arc::clone(&is_streaming);
        thread::spawn(move || loop {
            let time = Instant::now();
            let value = stream_manager.get_frame_as_base64();
            // emit current frame
            if window.emit("update_frame", value).is_err() {
                println!("failed to emit event");
            }
            println!("time : {:?}", time.elapsed());
            if let Ok(is_streaming) = cloned_is_streaming.lock() {
                if !*is_streaming {
                    println!("stream stopped");
                    break;
                }
            }
        });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_frame])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
