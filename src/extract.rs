use image::{GenericImage, ImageFormat};

const CAMERA_RESOLUTION: (u32, u32) = (1280, 720);

fn image_to_bytes(frame: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut bytes = Vec::new();
    let _ = frame
        .write_to(&mut std::io::Cursor::new(&mut bytes), ImageFormat::Jpeg)
        .unwrap();
    bytes
}

pub fn get_placement(
    llm_model: &mut llamacpp_embed::LlamaEmbedModel,
    llm_training_data: &[llamacpp_embed::VisionMessage],
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<u32> {
    let place_frame = frame
        .sub_image(
            (0.8368f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
            (0.7791f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
            (0.1289f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
            (0.1944f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
        )
        .to_image();
    //let _ = place_frame.save(&format!("test_place.png"));
    let place = crate::llm::identify(
        llm_model,
        &crate::llm::placement_prompt(),
        &image_to_bytes(&place_frame),
        llm_training_data,
    );
    crate::data::string_to_number(&place)
}

pub fn get_first_item(
    llm_model: &mut llamacpp_embed::LlamaEmbedModel,
    llm_training_data: &[llamacpp_embed::VisionMessage],
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<String> {
    let first_item_frame = frame
        .sub_image(
            (0.8368f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
            (0.7791f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
            (0.1289f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
            (0.1944f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
        )
        .to_image();
    //let _ = first_item_frame.save(&format!("test_first.png"));
    Some(crate::llm::identify(
        llm_model,
        &crate::llm::item_prompt(),
        &image_to_bytes(&first_item_frame),
        llm_training_data,
    ))
}

pub fn get_second_item(
    llm_model: &mut llamacpp_embed::LlamaEmbedModel,
    llm_training_data: &[llamacpp_embed::VisionMessage],
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<String> {
    let second_item_frame = frame
        .sub_image(
            (0.0273f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
            (0.0472f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
            (0.0540f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
            (0.0958f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
        )
        .to_image();
    //let _ = second_item_frame.save(&format!("test_second.png"));
    Some(crate::llm::identify(
        llm_model,
        &crate::llm::item_prompt(),
        &image_to_bytes(&second_item_frame),
        llm_training_data,
    ))
}

pub fn get_coin_count(
    engine: &ocrs::OcrEngine,
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<u32> {
    let coin_frame = frame
        .sub_image(
            (0.0601f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
            (0.9028f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
            (0.0414f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
            (0.0514f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
        )
        .to_image();
    //let _ = coin_frame.save(&format!("test_coin.png"));
    let coins = crate::ocr::extract_text(&engine, &coin_frame)?;
    crate::data::string_to_number(&coins)
}
