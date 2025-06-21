mod capture;
mod extract;
mod ocr;
mod phash;
mod twitch;

fn main() {
    // Initializations
    let ocr_engine = ocr::init();
    let mut camera: nokhwa::Camera = capture::init();
    let hasher = phash::init();

    // Capture and state-parsing
    loop {
        let mut frame = capture::get_video_frame(&mut camera);
        //let _ = frame.save(&format!("test_full.png"));
        let place = extract::get_placement(&hasher, &mut frame);
        let first_item = extract::get_first_item(&hasher, &mut frame);
        let second_item = extract::get_second_item(&hasher, &mut frame);
        let coin_count = extract::get_coin_count(&ocr_engine, &mut frame);
        println!(
            "State:\n\tPlace: {:?}\n\tFirst: {:?}\n\tSecond: {:?}\n\tCoins: {:?}",
            place,
            first_item,
            second_item,
            coin_count,
        );
        std::thread::sleep(std::time::Duration::from_secs(5)); // real looping
        //let _ = prompted::input!("NEXT"); // manual testing
    } // TODO: multi-thread this, plus output, etc

    // Twitch chat handling
    twitch::run();
}