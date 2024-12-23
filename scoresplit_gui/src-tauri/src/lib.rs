extern crate scoresplit_core;

use scoresplit_core::stream_manager::StreamManager;
use scoresplit_core::video_manager::VideoManager;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tauri::{window, AppHandle, Emitter, Listener, Manager, Window};
use tauri_plugin_dialog::DialogExt;

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

#[tauri::command]
/// open file dialog, and open video
fn open_video(window: Window, video_path: String) {
    println!("{:?}", video_path);

    if let Ok(video_manager) = VideoManager::new(video_path.as_str()) {
        let video_length = video_manager.get_video_length();
        let arc_video_manager = Arc::new(Mutex::new(video_manager));
        // add listner for seek event

        let cloned_video_manager = Arc::clone(&arc_video_manager);

        window.listen("video_seek", move |event| {
            println!("{:?}", event.payload());
            // remove '"' from string
            let mut payload = event.payload().to_string();
            payload.retain(|c| c != '"');
            // convert string number to int
            if let Ok(seek_value) = payload.parse::<i32>() {
                // convert seekbar value (0 ~ 100_000) to video length
                let seek_pos = video_length * (seek_value as f64 / 100000.0);
                if let Ok(mut video_manager) = cloned_video_manager.lock() {
                    video_manager.seek(seek_pos);
                }
            }
        });

        // close current video if opened new video
        let is_opened = Arc::new(Mutex::new(true));
        let cloned_is_opened = Arc::clone(&is_opened);
        window.listen("open_video", move |event| {
            if let Ok(mut is_opened) = cloned_is_opened.lock() {
                *is_opened = false;
            }
        });

        let cloned_video_manager = Arc::clone(&arc_video_manager);
        let cloned_is_opened = Arc::clone(&is_opened);
        // start video frame update loop
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(100));
            if let Ok(mut video_manager) = cloned_video_manager.lock() {
                let value = video_manager.get_current_frame_as_base64();
                if window.emit("update_frame", value).is_err() {
                    println!("failed to emit event");
                }
            }
            if let Ok(is_opened) = cloned_is_opened.lock() {
                if !(*is_opened) {
                    println!("video {:?} closed", video_path);
                    break;
                }
            }
        });
    } else {
        println!("failed to load video");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_frame, open_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
