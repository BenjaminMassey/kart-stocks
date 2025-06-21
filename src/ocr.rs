pub fn init() -> ocrs::OcrEngine {
    let detection_model_path = std::path::PathBuf::from("./data/ocr_models/text-detection.rten");
    let rec_model_path = std::path::PathBuf::from("./data/ocr_models/text-recognition.rten");

    let detection_model = rten::Model::load_file(detection_model_path).unwrap();
    let recognition_model = rten::Model::load_file(rec_model_path).unwrap();

    ocrs::OcrEngine::new(ocrs::OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    }).unwrap()
}

pub fn extract_text(
    engine: &ocrs::OcrEngine,
    frame: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>
) -> Option<String> {
    let img_source = ocrs::ImageSource::from_bytes(frame.as_raw(), frame.dimensions());
    if img_source.is_err() {
        return None;
    }
    let ocr_input = engine.prepare_input(img_source.unwrap());
    if ocr_input.is_err() {
        return None;
    }

    let result = engine.get_text(&ocr_input.unwrap());
    if let Ok(text) = result {
        return Some(text);
    }
    None
}