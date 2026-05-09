use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub twitch: Twitch,
    pub llm: Llm,
    pub obs: Obs,
    pub game: Game,
}

#[derive(Clone, Deserialize)]
pub struct Twitch {
    pub bot_channel: String,
    pub racer_channel: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_port: u16,
    pub debug_messages: bool,
}

#[derive(Clone, Deserialize)]
pub struct Llm {
    pub cycle_time: i32,
    pub port: u64,
    pub model_path: String,
    pub mmproj_path: String,
}

#[derive(Clone, Deserialize)]
pub struct Obs {
    pub ip: String,
    pub port: u16,
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct Game {
    pub database_path: String,
    pub starting_money: i32,
    pub initial_price: i32,
    pub base_price: f32,
    pub place_target: i32,
    pub volatility: f32,
    pub confidence: f32,
    pub place_weight: f32,
    pub items_weight: f32,
    pub coins_weight: f32,
}

pub fn get_settings() -> Settings {
    let contents = std::fs::read_to_string("settings.toml").expect("Failure to read settings.toml");
    toml::from_str(&contents).expect("Failure to parse settings.toml into Settings")
}
