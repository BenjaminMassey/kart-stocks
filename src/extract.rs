use std::collections::HashMap;

pub fn get_placement(
    hasher: &(img_hash::Hasher, HashMap<String, HashMap<String, img_hash::ImageHash>>),
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<String> {
    let place_frame = crate::capture::get_placement_image(frame);
    //let _ = place_frame.save(&format!("test_place.png"));
    let place = crate::phash::get_closest(&hasher.0, &place_frame, &hasher.1["places"]);
    Some(place) // TODO: threshold / status for None
    // TODO: is OCR better for placement?
}

pub fn get_first_item(
    hasher: &(img_hash::Hasher, HashMap<String, HashMap<String, img_hash::ImageHash>>),
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<String> {
    let first_item_frame = crate::capture::get_first_item_image(frame);
    //let _ = first_item_frame.save(&format!("test_first.png"));
    let first_item = crate::phash::get_closest(&hasher.0, &first_item_frame, &hasher.1["firsts"]);
    Some(first_item) // TODO: threshold / status for None
}

pub fn get_second_item(
    hasher: &(img_hash::Hasher, HashMap<String, HashMap<String, img_hash::ImageHash>>),
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<String> {
    let second_item_frame = crate::capture::get_second_item_image(frame);
    //let _ = second_item_frame.save(&format!("test_second.png"));
    let second_item = crate::phash::get_closest(&hasher.0, &second_item_frame, &hasher.1["seconds"]);
    Some(second_item) // TODO: threshold / status for None
}

pub fn get_coin_count(
    engine: &ocrs::OcrEngine,
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<String> {
    let coin_frame = crate::capture::get_coin_image(frame);
    //let _ = coin_frame.save(&format!("test_coin.png"));
    crate::ocr::extract_text(&engine, &coin_frame)
}