use base64::{engine::general_purpose, Engine};
use opencv::{
    core::{FileStorage, Image2DTraitConst, Mat, MatTraitConst, Vector, VectorToVec},
    highgui::{self, imshow, wait_key},
    imgcodecs::imencode,
    video,
    videoio::{
        self, VideoCapture, VideoCaptureTrait, VideoCaptureTraitConst, CAP_ANY, CAP_PROP_FPS,
        CAP_PROP_FRAME_COUNT, CAP_PROP_POS_FRAMES,
    },
};

use anyhow::Result;

pub struct VideoManager {
    cap: VideoCapture,
}

/// Controler for videos
impl VideoManager {
    /// create VideoManger struct from file path
    pub fn new(file_path: &str) -> Result<Self> {
        let cap = videoio::VideoCapture::from_file(file_path, CAP_ANY)?;
        Ok(VideoManager { cap })
    }
    /// seek to passed time, then update current frame
    pub fn seek(&mut self, time: f64) -> Result<()> {
        self.cap.set(CAP_PROP_POS_FRAMES, time)?;
        Ok(())
    }

    /// get video length(secs)
    pub fn get_video_length(&self) -> Result<f64> {
        let fps = self.cap.get(CAP_PROP_FPS)?;
        let frame_count = self.cap.get(CAP_PROP_FRAME_COUNT)?;
        // calc length(sec) from fps and frame count
        let length = frame_count / fps;
        Ok(length)
    }

    /// get current frame as base64 string
    pub fn get_current_frame_as_base64(&mut self) -> String {
        let mut current_frame = Mat::default();
        if self.cap.read(&mut current_frame).is_ok() {
            let mut buf = Vector::<u8>::default();
            let params = Vector::<i32>::default();
            // encode as jpg, then, convert the image to base64 string
            if imencode(".jpg", &current_frame, &mut buf, &params).is_ok() {
                let buf_vec = buf.to_vec();
                let res = general_purpose::STANDARD.encode(&buf_vec);
                return res;
            }
        }
        String::default()
    }
}

#[cfg(test)]
mod video_manager_test {
    use crate::video_manager::*;
    #[test]
    fn videomanager_construct_test() {
        let path = "/mnt/data/Videos/OBS_Records/2024-11-27 18-38-29.mp4";
        if let Ok(mut video_manager) = VideoManager::new(path) {
            let index = 60.0 * 42.0;
            video_manager.seek(index);
        }
    }

    #[test]
    fn get_frame_count_test() {
        let path = "/mnt/data/Videos/OBS_Records/2024-11-27 18-38-29.mp4";
        let video_manager = VideoManager::new(path).unwrap();
        let length = video_manager.get_video_length().unwrap();
        println!("length : {:?} (s)", length);
    }
}
