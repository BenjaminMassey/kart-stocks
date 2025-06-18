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
    let frame = capture::get_video_frame(&mut camera);
    let _ = frame.save("test.png");
    println!("Test frame saved to test.png.");

    println!("Starting Twitch client...");
    twitch::run();
}