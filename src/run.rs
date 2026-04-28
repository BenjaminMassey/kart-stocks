use std::collections::HashMap;

pub fn from_obs(
    password: &str,
    source: uuid::Uuid,
    state: &mut crate::data::State,
    hasher: &(
        img_hash::Hasher,
        HashMap<String, HashMap<String, img_hash::ImageHash>>,
    ),
    ocr_engine: &ocrs::OcrEngine,
) {
    let mut frame = crate::obs::get_obs_frame(password, source);
    update_state(&mut frame, state, hasher, ocr_engine);
}

pub fn from_camera(
    camera: &mut nokhwa::Camera,
    state: &mut crate::data::State,
    hasher: &(
        img_hash::Hasher,
        HashMap<String, HashMap<String, img_hash::ImageHash>>,
    ),
    ocr_engine: &ocrs::OcrEngine,
) {
    let mut frame = crate::capture::get_video_frame(camera);
    update_state(&mut frame, state, hasher, ocr_engine);
}

pub fn from_image(
    path: &str,
    state: &mut crate::data::State,
    hasher: &(
        img_hash::Hasher,
        HashMap<String, HashMap<String, img_hash::ImageHash>>,
    ),
    ocr_engine: &ocrs::OcrEngine,
) {
    let mut frame = image::open(path).unwrap().into_rgb8();
    update_state(&mut frame, state, hasher, ocr_engine);
}

fn update_state(
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    state: &mut crate::data::State,
    hasher: &(
        img_hash::Hasher,
        HashMap<String, HashMap<String, img_hash::ImageHash>>,
    ),
    ocr_engine: &ocrs::OcrEngine,
) {
    //let _ = frame.save(&format!("test_full.png"));
    if let Some(new_place) = crate::extract::get_placement(&hasher, frame) {
        state.place = new_place;
    }
    state.first_item = crate::extract::get_first_item(&hasher, frame);
    if state.first_item.is_some() {
        state.second_item = crate::extract::get_second_item(&hasher, frame);
    } else {
        state.second_item = None;
    }
    if let Some(new_coins) = crate::extract::get_coin_count(&ocr_engine, frame) {
        state.coin_count = new_coins;
    }
}
