use image::{GenericImage, ImageBuffer, Rgb};
use nokhwa::{pixel_format::RgbFormat, utils::*, Camera};

const CAMERA_INDEX_MAX: u32 = 10;
const CAMERA_RESOLUTION: (u32, u32) = (1280, 720);

pub fn get_list_cameras() -> Vec<Option<String>> {
    let mut names: Vec<Option<String>> = vec![];
    for i in 0..CAMERA_INDEX_MAX {
        let camera = get_camera(i);
        if let Some(cam) = camera {
            names.push(Some(cam.info().human_name()));
        } else {
            names.push(None);
        }
    }
    names
}

pub fn get_camera(index: u32) -> Option<Camera> {
    let index = CameraIndex::Index(index);
    let requested = RequestedFormat::new::<RgbFormat>(
        RequestedFormatType::HighestResolution(
            Resolution::new(CAMERA_RESOLUTION.0, CAMERA_RESOLUTION.1)
        )
    );
    if let Ok(cam) = Camera::new(index, requested) {
        return Some(cam);
    }
    None
}

pub fn get_video_frame(camera: &mut Camera) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let frame = camera.frame().unwrap();
    frame.decode_image::<RgbFormat>().unwrap()
}

pub fn get_placement_image(frame: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    frame.sub_image(
        (0.8368f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
        (0.7791f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
        (0.1289f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
        (0.1944f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
    ).to_image()
}

pub fn get_first_item_image(frame: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    frame.sub_image(
        (0.0906f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
        (0.0458f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
        (0.0977f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
        (0.1708f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
    ).to_image()
}

pub fn get_second_item_image(frame: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    frame.sub_image(
        (0.0273f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
        (0.0472f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
        (0.0540f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
        (0.0958f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
    ).to_image()
}

pub fn get_coin_image(frame: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    frame.sub_image(
        (0.0601f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
        (0.9028f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
        (0.0414f32 * (CAMERA_RESOLUTION.0 as f32)) as u32,
        (0.0514f32 * (CAMERA_RESOLUTION.1 as f32)) as u32,
    ).to_image()
}