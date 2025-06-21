use std::collections::HashMap;

const THRESHOLD: u32 = 22;

pub fn init() -> (img_hash::Hasher, HashMap<String, HashMap<String, img_hash::ImageHash>>) {
    let hasher = img_hash::HasherConfig::new().to_hasher();
    
    let mut master: HashMap<String, HashMap<String, img_hash::ImageHash>> = HashMap::new();
    master.insert("places".to_owned(), get_hashes(&hasher, "data/images/place"));
    master.insert("firsts".to_owned(), get_hashes(&hasher, "data/images/first"));
    master.insert("seconds".to_owned(), get_hashes(&hasher, "data/images/second"));

    (hasher, master)
}

fn get_hashes(
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
            map.insert(file_name[..file_name.len()-4].to_owned(), hash);
        }
    }
    map
}

pub fn get_closest(
    hasher: &img_hash::Hasher,
    frame: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    possibilities: &HashMap<String, img_hash::ImageHash>,
) -> Option<String> {
    let the_hash = hasher.hash_image(&hashable_convert(frame));
    let mut min = (String::new(), u32::MAX);
    for (key, value) in possibilities.iter() {
        let dist = the_hash.dist(value);
        if dist < min.1 {
            min = (key.clone(), dist);
        }
    }
    if min.1 >= THRESHOLD {
        return None;
    }
    Some(min.0)
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