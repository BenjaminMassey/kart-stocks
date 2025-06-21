use std::collections::HashMap;

pub fn get_placement(
    hasher: &(img_hash::Hasher, HashMap<String, HashMap<String, img_hash::ImageHash>>),
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<u32> {
    let place_frame = crate::capture::get_placement_image(frame);
    //let _ = place_frame.save(&format!("test_place.png"));
    let place = crate::phash::get_closest(&hasher.0, &place_frame, &hasher.1["places"]);
    crate::data::option_str_to_option_u32(place)
}

pub fn get_first_item(
    hasher: &(img_hash::Hasher, HashMap<String, HashMap<String, img_hash::ImageHash>>),
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<String> {
    let first_item_frame = crate::capture::get_first_item_image(frame);
    let _ = first_item_frame.save(&format!("test_first.png"));
    crate::phash::get_closest(&hasher.0, &first_item_frame, &hasher.1["firsts"])
}

pub fn get_second_item(
    hasher: &(img_hash::Hasher, HashMap<String, HashMap<String, img_hash::ImageHash>>),
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<String> {
    let second_item_frame = crate::capture::get_second_item_image(frame);
    let _ = second_item_frame.save(&format!("test_second.png"));
    crate::phash::get_closest(&hasher.0, &second_item_frame, &hasher.1["seconds"])
}

pub fn get_coin_count(
    engine: &ocrs::OcrEngine,
    frame: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Option<u32> {
    let coin_frame = crate::capture::get_coin_image(frame);
    //let _ = coin_frame.save(&format!("test_coin.png"));
    let coins = crate::ocr::extract_text(&engine, &coin_frame);
    crate::data::option_str_to_option_u32(coins)
}