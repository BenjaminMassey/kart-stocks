use std::collections::HashMap;

pub fn get_hashes(
    hasher: &img_hash::Hasher,
    folder: &str
) -> HashMap<String, img_hash::ImageHash> {
    let mut map: HashMap<String, img_hash::ImageHash> = HashMap::new();
    let entries = std::fs::read_dir("./".to_owned() + folder).unwrap();
    for entry in entries {
        if let Ok(file) = entry {
            let file_name = file.file_name().into_string().unwrap();
            let file_path = format!("./{folder}/{file_name}");
            let frame = image::open(file_path).unwrap();
            let hash = hasher.hash_image(&hashable_convert(&frame.to_rgb8()));
            map.insert(file_name, hash);
        }
    }
    map
}

pub fn get_closest(
    hasher: &img_hash::Hasher,
    frame: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    possibilities: &HashMap<String, img_hash::ImageHash>,
) -> String {
    let the_hash = hasher.hash_image(&hashable_convert(frame));
    let mut min = (String::new(), u32::MAX);
    for (key, value) in possibilities.iter() {
        let dist = the_hash.dist(value);
        if dist < min.1 {
            min = (key.clone(), dist);
        }
    }
    min.0
}

fn hashable_convert(
    frame: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> img_hash::image::DynamicImage {
    img_hash::image::DynamicImage::ImageRgb8(
        img_hash::image::ImageBuffer::from_raw(
            frame.width(),
            frame.height(),
            frame.as_raw().to_vec()
        ).unwrap()
    )
}