use kart_stocks::{llm, settings};

fn main() {
    println!("Starting kart_stocks LLM server...");
    let mut settings = settings::get_settings();
    settings.llm.client_only = false;
    let mut llm_model = llm::init(&settings);
    let _ = prompted::input!("Loaded! Press enter to stop.");
    llamacpp_embed::stop(&mut llm_model).unwrap();
}
