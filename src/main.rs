mod capture;
mod extract;
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

    let hasher = img_hash::HasherConfig::new().to_hasher();

    let places = extract::get_hashes(&hasher, "data/place");
    let firsts = extract::get_hashes(&hasher, "data/first");
    let seconds = extract::get_hashes(&hasher, "data/second");


    loop {
        let mut frame = capture::get_video_frame(&mut camera);
        //let _ = frame.save(&format!("test_full.png"));

        let place_frame = capture::get_placement_image(&mut frame);
        //let _ = place_frame.save(&format!("test_place.png"));
        let place = extract::get_closest(&hasher, &place_frame, &places);
        println!("Placement: {place}");

        let first_item_frame = capture::get_first_item_image(&mut frame);
        //let _ = first_item_frame.save(&format!("test_first.png"));
        let first_item = extract::get_closest(&hasher, &first_item_frame, &firsts);
        println!("First Item: {first_item}");

        let second_item_frame = capture::get_second_item_image(&mut frame);
        //let _ = second_item_frame.save(&format!("test_second.png"));
        let second_item = extract::get_closest(&hasher, &second_item_frame, &seconds);
        println!("Second Item: {second_item}");

        //let coin_frame = capture::get_coin_image(&mut frame);
        //let _ = coin_frame.save(&format!("test_coin.png"));

        std::thread::sleep(std::time::Duration::from_secs(5));
    } // TODO: multi-thread this, plus output, plus coins, etc

    println!("Test frames saved.");

    println!("Starting Twitch client...");
    twitch::run();
}