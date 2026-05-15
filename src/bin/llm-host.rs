use kart_stocks::{llm, settings};

fn main() {
    println!("Starting kart_stocks LLM server...");
    let settings = settings::get_settings();
    let mut llm_model = llm::init(&settings);
    llm::prep_training_data(&mut llm_model);
    let _ = prompted::input!("Loaded! Press enter to stop.");
    llamacpp_embed::stop(&mut llm_model).unwrap();
}
