mod capture;
mod data;
mod extract;
mod ocr;
mod phash;
mod twitch;

fn main() {
    // Initializations
    let ocr_engine = ocr::init();
    let hasher = phash::init();
    let mut camera: nokhwa::Camera = capture::init();

    // Capture and state-parsing
    let mut state = data::State::new();
    loop {
        let mut frame = capture::get_video_frame(&mut camera);
        //let _ = frame.save(&format!("test_full.png"));
        if let Some(new_place) = extract::get_placement(&hasher, &mut frame) {
            state.place = new_place;
        }
        state.first_item = extract::get_first_item(&hasher, &mut frame);
        if state.first_item.is_some() {
            state.second_item = extract::get_second_item(&hasher, &mut frame);
        } else {
            state.second_item = None;
        }
        if let Some(new_coins) = extract::get_coin_count(&ocr_engine, &mut frame) {
            state.coin_count = new_coins;
        }
        println!("State:\n\t{state:?}");
        std::thread::sleep(std::time::Duration::from_secs(5)); // real looping
        //let _ = prompted::input!("NEXT"); // manual testing
    } // TODO: multi-thread this, plus output, etc

    // Twitch chat handling
    twitch::run();
}