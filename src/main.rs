mod capture;
mod twitch;

fn main() {
    let cameras = capture::get_list_cameras();
    if cameras.is_empty() {
        panic!("No camera devices found.");
    }
    println!("Possible camera devices by index:");
    for (i, camera) in cameras.iter().enumerate() {
        if let Some(cam) = camera {
            println!("\t{}: {}", i, cam);
        }
    }

    let index_input = prompted::input!("Index selection: ");
    let index: u32 = match index_input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid index input."),
    };

    let mut camera: nokhwa::Camera = capture::get_camera(index).expect("Failed to get capture.");

    let mut frame = capture::get_video_frame(&mut camera);
    let _ = frame.save(&format!("test_full.png"));

    let place = capture::get_placement_image(&mut frame);
    let _ = place.save(&format!("test_place.png"));

    let first_item = capture::get_first_item_image(&mut frame);
    let _ = first_item.save(&format!("test_first.png"));

    let second_item = capture::get_second_item_image(&mut frame);
    let _ = second_item.save(&format!("test_second.png"));

    let coin = capture::get_coin_image(&mut frame);
    let _ = coin.save(&format!("test_coin.png"));

    println!("Test frames saved.");

    println!("Starting Twitch client...");
    twitch::run();
}