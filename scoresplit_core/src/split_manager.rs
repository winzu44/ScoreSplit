use std::time::Instant;

use anyhow::{anyhow, Context, Ok, Result};
use base64::{engine::general_purpose, Engine};
use ocrs::{ImageSource, OcrEngine, OcrEngineParams, TextLine};
use opencv::core::{min_max_loc, Mat, MatTraitConst, MatTraitConstManual, Point, Rect};
use opencv::imgcodecs::{imdecode, IMREAD_GRAYSCALE};
use opencv::imgproc::{match_template, TM_CCORR_NORMED};

use rten::Model;
#[derive(Debug)]
struct ScoreLocation {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Debug)]
struct Split {
    trigger_image: String,
    score_location: ScoreLocation,
}
struct SplitManager {
    split_index: usize,
    splits: Vec<Split>,
    threshold: f64,
    ocr_engine: OcrEngine,
}

// convert .jpg decoded base64 string to OpenCV Mat
fn base64_to_Mat(base64_str: &str) -> Result<Mat> {
    // convert base64 string to Vec<u8>
    let image_vec = general_purpose::STANDARD.decode(base64_str)?;
    let input_mat = Mat::from_bytes::<u8>(&image_vec)?;
    // then decode base64 string to Mat
    let image = imdecode(&input_mat, IMREAD_GRAYSCALE)?;
    Ok(image)
}

impl SplitManager {
    /// create split manager object from .ssplt file
    /// .ssplt file contains : trigger image (as base64 string), location, target position and so on...
    pub fn new() -> Self {
        // prepare ocrs engine
        let detection_model = Model::load_file("models/text-detection.rten").unwrap();
        let rec_model = Model::load_file("models/text-recognition.rten").unwrap();
        let params = OcrEngineParams {
            detection_model: Some(detection_model),
            recognition_model: Some(rec_model),
            ..Default::default()
        };
        let ocr_engine = OcrEngine::new(params).unwrap();
        SplitManager {
            split_index: 0,
            splits: vec![],
            threshold: 0.95,
            ocr_engine,
        }
    }
    /// check if current target exits
    /// if target exists, return value of score
    /// * 'target_image' - base64 string of target image (.jpg)
    pub fn check(&self, target_image: &str) -> Option<i32> {
        let res = || -> Result<i32> {
            let target_image = base64_to_Mat(target_image)?;
            // get current split, and covert trigger image string to Mat
            let current_split = self
                .splits
                .get(self.split_index)
                .context("no current split found")?;
            let trigger_image = base64_to_Mat(current_split.trigger_image.as_str())?;
            // apply template matcing
            let mut result = Mat::default();
            match_template(
                &target_image,
                &trigger_image,
                &mut result,
                TM_CCORR_NORMED,
                &trigger_image,
            )?;

            // get maching result
            let mask = Mat::default();
            let mut min = 0.0;
            let mut max = 0.0;
            let mut min_pointer = Point::default();
            let mut max_pointer = Point::default();
            min_max_loc(
                &result,
                Some(&mut min),
                Some(&mut max),
                Some(&mut min_pointer),
                Some(&mut max_pointer),
                &mask,
            )?;
            // max value means percentage of if target exists
            if max > self.threshold {
                // if exists, get score value on selected location
                // crop image for score location
                let loc = &current_split.score_location;
                let rect = Rect::new(loc.x, loc.y, loc.width, loc.height);
                let cropped_image = Mat::roi(&target_image, rect)?.try_clone()?;

                let image_source = ImageSource::from_bytes(
                    cropped_image.data_bytes()?,
                    (loc.width as u32, loc.height as u32),
                )?;

                // apply ocr process using ocrs engine
                let ocr_input = self.ocr_engine.prepare_input(image_source)?;
                let word_rects = self.ocr_engine.detect_words(&ocr_input)?;

                let line_rects = self.ocr_engine.find_text_lines(&ocr_input, &word_rects);
                let line_texts = self.ocr_engine.recognize_text(&ocr_input, &line_rects)?;
                // accepts only line_text.len() == 1
                if line_texts.iter().len() != 1 {
                    return Err(anyhow!("multiple line found"));
                }

                // convert line text to i32 value
                if let Some(line) = line_texts
                    .iter()
                    .flatten()
                    .find(|l| l.to_string().len() > 1)
                {
                    let line_string = line.to_string();
                    let value = line_string.parse::<i32>()?;
                    return Ok(value);
                }

                return Err(anyhow!("failed to convert line string to value"));
            }
            Err(anyhow!("no target found"))
        };
        if res().is_ok() {
            let value = res().unwrap();
            return Some(value);
        }

        None
    }

    /// add new split
    /// * 'target_image' - base64 string of target image (.jpg)
    /// * 'trigger_location' - locatoin of trigger image (left corner)
    /// * 'score_location' - locatoin of score (left corner)
    pub fn add_split(&mut self, trigger_image: &str, score_location: ScoreLocation) {
        let split = Split {
            trigger_image: trigger_image.to_string(),
            score_location,
        };
        self.splits.push(split);
    }
    /// open .ssplt file from path
    pub fn open(path: &str) {}
    /// generate .ssplt file from current SplitManager object
    pub fn save() {}
}

#[cfg(test)]
mod split_manager_test {
    use std::time::Instant;

    use opencv::{
        core::{Mat, Vector, VectorToVec},
        imgcodecs::{imencode, imread, IMREAD_GRAYSCALE},
    };

    use super::{ScoreLocation, SplitManager};
    use base64::{engine::general_purpose, Engine};
    fn Mat_to_base64(image: Mat) -> String {
        let mut buf = Vector::<u8>::default();
        let mut params = Vector::<i32>::default();
        // convert image to base64 string
        let res = imencode(".jpg", &image, &mut buf, &params).unwrap();

        // add new split
        let mut buf_vec = buf.to_vec();
        general_purpose::STANDARD.encode(&buf_vec)
    }

    #[test]
    fn split_test() {
        let mut split_manager = SplitManager::new();
        let target_image = imread("sample_data/target_act.png", IMREAD_GRAYSCALE).unwrap();

        let image_string = Mat_to_base64(target_image);
        let score_location = ScoreLocation {
            x: 123,
            y: 166,
            width: 473,
            height: 66,
        };
        split_manager.add_split(image_string.as_str(), score_location);

        let trigger_image = imread("sample_data/test1.png", IMREAD_GRAYSCALE).unwrap();
        // let trigger_image = imread("sample_data/target_act.png", IMREAD_GRAYSCALE).unwrap();
        let trigger_string = Mat_to_base64(trigger_image);
        let time = Instant::now();
        let val = split_manager.check(trigger_string.as_str());
        println!("{:?}", val);
        println!("{:?}", time.elapsed());
    }
}
