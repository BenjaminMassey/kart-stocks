pub fn from_obs(
    password: &str,
    source: uuid::Uuid,
    state: &mut crate::data::State,
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
    state: &mut crate::data::State,
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
    state: &mut crate::data::State,
    llm_model: &mut llamacpp_embed::LlamaEmbedModel,
    llm_placement_data: &[llamacpp_embed::VisionMessage],
    llm_item_data: &[llamacpp_embed::VisionMessage],
    ocr_engine: &ocrs::OcrEngine,
) {
    //let _ = frame.save(&format!("test_full.png"));
    if let Some(new_place) = crate::extract::get_placement(llm_model, llm_placement_data, frame) {
        state.place = new_place;
    }
    state.first_item = crate::extract::get_first_item(llm_model, llm_item_data, frame);
    if state.first_item.is_some() {
        state.second_item = crate::extract::get_second_item(llm_model, llm_item_data, frame);
    } else {
        state.second_item = None;
    }
    if let Some(new_coins) = crate::extract::get_coin_count(&ocr_engine, frame) {
        state.coin_count = new_coins;
    }
}
