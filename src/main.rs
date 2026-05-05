mod data;
mod extract;
mod llm;
mod obs;
mod ocr;
mod run;
mod twitch;

const CYCLE_TIME: u64 = 5;

fn main() {
    println!("Starting kart-stocks...");
    // Initializations
    let ocr_engine = ocr::init();
    let mut llm_model = llm::init();
    let llm_placement_data = llm::get_placement_data();
    let llm_item_data = llm::get_item_data();
    let obws_password = obs::get_obws_password();
    println!("\nFinished initializing!\n\nPlease choose your OBS source.\n");
    let obws_source = obs::choose_obs_source(&obws_password);
    println!("\nSetup complete: doing initial prompting, followed by runs.\n");

    // Capture and state-parsing
    let mut state = data::State::new();
    loop {
        run::from_obs(
            &obws_password,
            obws_source,
            &mut state,
            &mut llm_model,
            &llm_placement_data,
            &llm_item_data,
            &ocr_engine,
        );
        let value = state.value();
        println!("{state:?} => {value}");

        std::thread::sleep(std::time::Duration::from_secs(
            0i32.max(CYCLE_TIME as i32 - state.time.elapsed().as_secs() as i32) as u64,
        ));
    } // TODO: multi-thread this with twitch::run()

    //llamacpp_embed::stop(&mut llm_model).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn images_directory() {
        println!("Initializing OCR engine...");
        let ocr_engine = ocr::init();
        println!("Initializing LLM...");
        let mut llm_model = llm::init();
        let llm_placement_data = llm::get_placement_data();
        let llm_item_data = llm::get_item_data();
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
                run::from_image(
                    path.to_str().unwrap(),
                    &mut state,
                    &mut llm_model,
                    &llm_placement_data,
                    &llm_item_data,
                    &ocr_engine,
                );
                let coins: u32 = name_pieces[0].parse().unwrap();
                let placement: u32 = name_pieces[1].parse().unwrap();
                let first_item: String = name_pieces[2].trim().to_owned();
                let second_item: String = name_pieces[3].trim().trim_end_matches(".png").to_owned();
                correct[0] = state.coin_count == coins;
                correct[1] = state.place == placement;
                correct[2] = state.first_item == first_item;
                correct[3] = state.second_item == second_item;
                println!(
                    "\tCoins: {} ({})\n\tPlace: {} ({})\n\tItem1: {} ({:?})\n\tItem2: {} ({:?})\n",
                    correct[0],
                    state.coin_count,
                    correct[1],
                    state.place,
                    correct[2],
                    state.first_item,
                    correct[3],
                    state.second_item,
                )
            }
        }
        llamacpp_embed::stop(&mut llm_model).unwrap();
    } // TODO: replce correct area with asserts and remove prints
}
