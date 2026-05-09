use std::sync::{Arc, Mutex};

const CYCLE_TIME: u64 = 5;

pub fn state_loop(
    obws_password: &str,
    obws_source: uuid::Uuid,
    state: Arc<Mutex<crate::data::State>>,
    llm_model: &mut llamacpp_embed::LlamaEmbedModel,
    llm_placement_data: &[llamacpp_embed::VisionMessage],
    llm_item_data: &[llamacpp_embed::VisionMessage],
    ocr_engine: &ocrs::OcrEngine,
) {
    loop {
        if state.lock().unwrap().racing {
            crate::run::from_obs(
                obws_password,
                obws_source,
                state.clone(),
                llm_model,
                llm_placement_data,
                llm_item_data,
                ocr_engine,
            );
        }
        state.lock().unwrap().time = std::time::Instant::now();
        state.lock().unwrap().update_value();
        println!("{:?}", state.lock().unwrap());

        if !state.lock().unwrap().running {
            break;
        }

        let elapsed = state.lock().unwrap().time.elapsed().as_secs();
        {
            std::thread::sleep(std::time::Duration::from_secs(
                0i32.max(CYCLE_TIME as i32 - elapsed as i32) as u64,
            ));
        }
    }
    llamacpp_embed::stop(llm_model).unwrap();
}

fn from_obs(
    password: &str,
    source: uuid::Uuid,
    state: Arc<Mutex<crate::data::State>>,
    llm_model: &mut llamacpp_embed::LlamaEmbedModel,
    llm_placement_data: &[llamacpp_embed::VisionMessage],
    llm_item_data: &[llamacpp_embed::VisionMessage],
    ocr_engine: &ocrs::OcrEngine,
) {
    let mut frame = crate::obs::get_obs_frame(password, source);
    update_state(
        &mut frame,
        state,
        llm_model,
        llm_placement_data,
        llm_item_data,
        ocr_engine,
    );
}

pub fn from_image(
    path: &str,
    state: Arc<Mutex<crate::data::State>>,
    llm_model: &mut llamacpp_embed::LlamaEmbedModel,
    llm_placement_data: &[llamacpp_embed::VisionMessage],
    llm_item_data: &[llamacpp_embed::VisionMessage],
    ocr_engine: &ocrs::OcrEngine,
) {
    let mut frame = image::open(path).unwrap().into_rgb8();
    update_state(
        &mut frame,
        state,
        llm_model,
        llm_placement_data,
        llm_item_data,
        ocr_engine,
    );
}

fn update_state(
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    state: Arc<Mutex<crate::data::State>>,
    llm_model: &mut llamacpp_embed::LlamaEmbedModel,
    llm_placement_data: &[llamacpp_embed::VisionMessage],
    llm_item_data: &[llamacpp_embed::VisionMessage],
    ocr_engine: &ocrs::OcrEngine,
) {
    let mut new_state = state.lock().unwrap().clone();
    //let _ = frame.save(&format!("test_full.png"));
    if let Some(new_place) = crate::extract::get_placement(llm_model, llm_placement_data, frame) {
        new_state.place = new_place;
    }
    let new_first_item = crate::extract::get_first_item(llm_model, llm_item_data, frame);
    if crate::data::valid_item(&new_first_item) {
        new_state.first_item = new_first_item;
    }
    let new_second_item = crate::extract::get_second_item(llm_model, llm_item_data, frame);
    if crate::data::valid_item(&new_second_item) {
        new_state.second_item = new_second_item;
    }
    if let Some(new_coins) = crate::extract::get_coin_count(ocr_engine, frame) {
        if new_coins <= 20 {
            new_state.coin_count = new_coins;
        }
    }
    *state.lock().unwrap() = new_state;
}
