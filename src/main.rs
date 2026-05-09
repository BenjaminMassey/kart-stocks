mod data;
mod extract;
mod llm;
mod obs;
mod ocr;
mod portfolio;
mod run;
mod twitch;
mod twitch_auth;

use std::sync::{Arc, Mutex};

fn main() {
    println!("Starting kart-stocks...");
    portfolio::init().unwrap();
    let ocr_engine = ocr::init();
    let mut llm_model = llm::init();
    let llm_placement_data = llm::get_placement_data();
    let llm_item_data = llm::get_item_data();
    let obws_password = obs::get_obws_password();
    println!("\nFinished initializing!\n\nPlease choose your OBS source.\n");
    let obws_source = obs::choose_obs_source(&obws_password);
    let twitch_token = twitch_auth::fetch_token();
    println!("\nSetup complete: doing initial prompting, followed by runs.\n");

    let state = Arc::new(Mutex::new(data::State::new()));
    let (send_to_twitch, receive_from_hotkey) = tokio::sync::mpsc::unbounded_channel::<String>();

    let state_for_updates = Arc::clone(&state);
    let state_thread = std::thread::spawn(move || {
        run::state_loop(
            &obws_password,
            obws_source,
            state_for_updates,
            &mut llm_model,
            &llm_placement_data,
            &llm_item_data,
            &ocr_engine,
        );
    });

    let hotkey_hook = livesplit_hotkey::Hook::new().unwrap();
    let hotkey = livesplit_hotkey::Hotkey {
        key_code: livesplit_hotkey::KeyCode::KeyK,
        modifiers: livesplit_hotkey::Modifiers::CONTROL | livesplit_hotkey::Modifiers::ALT,
    };
    let state_for_hotkey = Arc::clone(&state);
    hotkey_hook
        .register(hotkey, move || {
            let mut state = state_for_hotkey.lock().unwrap();
            state.racing = !state.racing;
            println!("{}ed racing!", if state.racing { "Start" } else { "Stopp" });
            if state.racing {
                state.race_start_time = std::time::Instant::now();
                send_to_twitch
                    .send("Starting race!".to_owned())
                    .expect("Error sending from hotkey to twitch.");
            } else {
                send_to_twitch
                    .send(format!("Selling to all investors at ${}.", state.value))
                    .expect("Error sending from hotkey to twitch.");
                if let Err(e) = portfolio::sell_all(state.value) {
                    eprintln!("{:?}", e);
                }
            }
        })
        .unwrap();

    twitch::run(Arc::clone(&state), receive_from_hotkey, &twitch_token);

    let _ = state_thread.join();

    println!("Goodbye!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn images_directory() {
        println!("Starting test run...");
        let state = Arc::new(Mutex::new(data::State::new()));
        println!("Initializing OCR engine...");
        let ocr_engine = ocr::init();
        println!("Initializing LLM...");
        let mut llm_model = llm::init();
        let llm_placement_data = llm::get_placement_data();
        let llm_item_data = llm::get_item_data();
        println!("Done initializing.");
        for item in std::fs::read_dir("./test/").unwrap() {
            if let Ok(file) = item {
                let mut correct = [false, false, false, false];
                let name = file.file_name().into_string().unwrap();
                println!("Testing {}...", &name);
                let name_pieces: Vec<String> = name.split("_").map(|s| s.to_owned()).collect();
                assert_eq!(name_pieces.len(), 4);
                let path = file.path();
                run::from_image(
                    path.to_str().unwrap(),
                    state.clone(),
                    &mut llm_model,
                    &llm_placement_data,
                    &llm_item_data,
                    &ocr_engine,
                );
                let coins: u32 = name_pieces[0].parse().unwrap();
                let placement: String = name_pieces[1].trim().to_owned();
                let first_item: String = name_pieces[2].trim().to_owned();
                let second_item: String = name_pieces[3].trim().trim_end_matches(".png").to_owned();
                let s = state.lock().unwrap();
                correct[0] = s.coin_count == coins;
                correct[1] = s.place.to_string() == placement;
                correct[2] = s.first_item == first_item;
                correct[3] = s.second_item == second_item;
                println!(
                    "\tCoins: {} ({})\n\tPlace: {} ({})\n\tItem1: {} ({:?})\n\tItem2: {} ({:?})\n",
                    correct[0],
                    s.coin_count,
                    correct[1],
                    s.place,
                    correct[2],
                    s.first_item,
                    correct[3],
                    s.second_item,
                )
            }
        }
        llamacpp_embed::stop(&mut llm_model).unwrap();
    }
}
