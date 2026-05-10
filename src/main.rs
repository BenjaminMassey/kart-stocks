mod data;
mod extract;
mod llm;
mod obs;
mod ocr;
mod portfolio;
mod run;
mod settings;
mod twitch;
mod twitch_auth;
mod window;

use std::sync::{Arc, Mutex};

const RESOLUTION: (f32, f32) = (1280.0, 720.0);

fn main() {
    println!("Starting kart-stocks...");
    let settings = settings::get_settings();
    portfolio::init(&settings).unwrap();
    let ocr_engine = ocr::init();
    let mut llm_model = llm::init(&settings);
    let llm_placement_data = llm::get_placement_data();
    let llm_item_data = llm::get_item_data();
    println!("\nFinished initializing!\n\nPlease choose your OBS source.\n");
    let obws_source = obs::choose_obs_source(&settings);
    let twitch_token = twitch_auth::fetch_token(&settings);
    println!("\nSetup complete: doing initial prompting, followed by runs.\n");

    let state = Arc::new(Mutex::new(data::State::new(&settings)));

    let (send_value_to_window, receive_from_run) = tokio::sync::mpsc::unbounded_channel::<i32>();
    let settings_for_run = settings.clone();
    let state_for_updates = Arc::clone(&state);
    let state_thread = std::thread::spawn(move || {
        run::state_loop(
            &settings_for_run,
            obws_source,
            state_for_updates,
            &mut llm_model,
            &llm_placement_data,
            &llm_item_data,
            &ocr_engine,
            send_value_to_window,
        );
    });

    let (send_actions_to_window, receive_from_twitch) =
        tokio::sync::mpsc::unbounded_channel::<twitch::InvestmentAction>();
    window::start(&settings, receive_from_run, receive_from_twitch);

    let (send_to_twitch, receive_from_hotkey) = tokio::sync::mpsc::unbounded_channel::<String>();
    let hotkey_hook = livesplit_hotkey::Hook::new().unwrap();
    let hotkey = livesplit_hotkey::Hotkey {
        key_code: livesplit_hotkey::KeyCode::KeyK,
        modifiers: livesplit_hotkey::Modifiers::CONTROL | livesplit_hotkey::Modifiers::ALT,
    };
    let settings_for_hotkey = settings.clone();
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
                let value = state.sell_all_price();
                send_to_twitch
                    .send(format!("Selling to all investors at ${}.", value))
                    .expect("Error sending from hotkey to twitch.");
                if let Err(e) = portfolio::sell_all(&settings_for_hotkey.clone(), value) {
                    eprintln!("{:?}", e);
                }
            }
        })
        .unwrap();

    twitch::run(
        &settings.clone(),
        Arc::clone(&state),
        receive_from_hotkey,
        &twitch_token,
        send_actions_to_window,
    );

    let _ = state_thread.join();

    println!("Goodbye!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn images_directory() {
        println!("Starting test run...");
        let settings = settings::get_settings();
        let state = Arc::new(Mutex::new(data::State::new(&settings)));
        println!("Initializing OCR engine...");
        let ocr_engine = ocr::init();
        println!("Initializing LLM...");
        let mut llm_model = llm::init(&settings);
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
