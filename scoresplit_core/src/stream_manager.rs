use base64::{engine::general_purpose, Engine};
use opencv::{
    core::{FileStorage, Image2DTraitConst, Mat, MatTraitConst, Vector, VectorToVec},
    imgcodecs::imencode,
    video,
    videoio::{self, VideoCapture, VideoCaptureTrait, VideoCaptureTraitConst, CAP_ANY},
};

/// Controller for streams (from stream device(e.g.capture card))
pub struct StreamManager {
    cam: VideoCapture,
}

impl StreamManager {
    pub fn new() -> Result<Self, ()> {
        let cam = videoio::VideoCapture::new(0, CAP_ANY);
        if let Ok(cam) = cam {
            Ok(StreamManager { cam })
        } else {
            Err(())
        }
    }

    /// get frame from current stream, then return image as base64 string
    pub fn get_frame_as_base64(&mut self) -> String {
        let mut current_frame = Mat::default();
        if self.cam.read(&mut current_frame).is_ok() {
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
#[test]
fn get_frame_test() {
    let mut mng = StreamManager::new();
    assert!(mng.is_ok());
    for _ in 0..100 {
        let res = mng.as_mut().unwrap().get_frame_as_base64();
        println!("{:?}", res);
    }
}
