mod data;
mod extract;
mod obs;
mod ocr;
mod phash;
mod run;
mod twitch;

fn main() {
    // Initializations
    let ocr_engine = ocr::init();
    let hasher = phash::init();
    let obws_password = obs::get_obws_password();
    let obws_source = obs::choose_obs_source(&obws_password);

    // Capture and state-parsing
    let mut state = data::State::new();
    loop {
        run::from_obs(
            &obws_password,
            obws_source,
            &mut state,
            &hasher,
            &ocr_engine,
        );
        println!("State:\n\t{state:?}");
        std::thread::sleep(std::time::Duration::from_secs(5));
        //let _ = prompted::input!("NEXT"); // manual testing
    } // TODO: multi-thread this with twitch::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn images_directory() {
        println!("Initializing OCR engine...");
        let ocr_engine = ocr::init();
        println!("Initializing image hasher...");
        let hasher = phash::init();
        println!("Done initializing.");
        for item in std::fs::read_dir("./test/images/").unwrap() {
            if let Ok(file) = item {
                let mut correct = [false, false, false, false];
                let name = file.file_name().into_string().unwrap();
                println!("Testing {}...", &name);
                let name_pieces: Vec<String> = name.split("_").map(|s| s.to_owned()).collect();
                assert_eq!(name_pieces.len(), 4);
                let path = file.path();
                let mut state = data::State::new();
                run::from_image(path.to_str().unwrap(), &mut state, &hasher, &ocr_engine);
                let coins: u32 = name_pieces[0].parse().unwrap();
                let placement: u32 = name_pieces[1].parse().unwrap();
                let first_item: String = name_pieces[2].clone();
                let second_item: String = name_pieces[3].clone();
                correct[0] = state.coin_count == coins;
                correct[1] = state.place == placement;
                if let Some(item) = state.first_item.clone() {
                    correct[2] = item == first_item;
                } else {
                    correct[2] = "none" == &first_item;
                }
                if let Some(item) = state.second_item.clone() {
                    let fixed_item = item.split(".").collect::<Vec<&str>>()[0];
                    correct[3] = fixed_item == &second_item;
                } else {
                    correct[3] = "none.png" == &second_item;
                }
                println!(
                    "\tCoins: {} ({})\n\tPlace: {} ({})\n\tItem1: {} ({:?})\n\tItem2: {} ({:?})\n",
                    correct[0],
                    state.coin_count,
                    correct[1],
                    state.place,
                    correct[2],
                    state.first_item.as_ref(),
                    correct[3],
                    state.second_item.as_ref(),
                )
            }
        }
    } // TODO: replce correct area with asserts and remove prints
}
