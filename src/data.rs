pub const RESOLUTION: (f32, f32) = (1280.0, 720.0);

pub const PLACEMENT_SLOT: u64 = 0;
pub const ITEMS_SLOT: u64 = 1;

static ITEM_VALUES: phf::Map<&'static str, i32> = phf::phf_map! {
    "none" => 0,
    "blooper" => 1,
    "blue-shell" => 2,
    "coin" => 5,
    "coin-block" => 5,
    "boomerang" => 5,
    "hammers" => 5,
    "kamek" => 10,
    "ice-flower" => 10,
    "fire-flower" => 10,
    "coin-shell" => 15,
    "banana" => 15,
    "green-shell" => 15,
    "feather" => 20,
    "horn" => 20,
    "bombomb" => 20,
    "food" => 20,
    "red-shell" => 20,
    "lightning" => 30,
    "double-banana" => 35,
    "double-green-shell" => 35,
    "double-red-shell" => 40,
    "mushroom" => 40,
    "triple-banana" => 45,
    "triple-green-shell" => 45,
    "triple-red-shell" => 50,
    "double-mushroom" => 55,
    "triple-mushroom" => 70,
    "golden-mushroom" => 85,
    "mega-mushroom" => 90,
    "boo" => 95,
    "star" => 95,
    "bullet-bill" => 100,
}; // should remain range from 0 through 100
pub fn valid_item(item: &str) -> bool {
    ITEM_VALUES.contains_key(item)
}
pub fn get_items() -> Vec<String> {
    ITEM_VALUES
        .keys()
        .map(|s| s.to_owned().to_owned())
        .collect()
}

#[derive(Clone)]
pub struct State {
    settings: crate::settings::Settings,
    pub running: bool,
    pub racing: bool,
    pub time: std::time::Instant,
    pub place: u32,
    pub first_item: String,
    pub second_item: String,
    pub coin_count: u32,
    pub race_start_time: std::time::Instant,
    pub value: i32,
    pub recent_buys: Vec<i32>,
    pub recent_sells: Vec<i32>,
}
impl State {
    pub fn new(settings: &crate::settings::Settings) -> Self {
        Self {
            settings: settings.clone(),
            running: true,
            racing: false,
            time: std::time::Instant::now(),
            place: 24,
            first_item: "none".to_owned(),
            second_item: "none".to_owned(),
            race_start_time: std::time::Instant::now(),
            coin_count: 0,
            value: 0,
            recent_buys: vec![],
            recent_sells: vec![],
        }
    }

    pub fn update_value(&mut self) {
        if self.racing {
            let place = (24 - self.place) as f32 / 23.0;
            let first_item = ITEM_VALUES[&self.first_item] as f32 / 100.0;
            let second_item = ITEM_VALUES[&self.second_item] as f32 / 100.0;
            let coin_count = self.coin_count as f32 / 20.0;
            let race_time = self.race_start_time.elapsed().as_secs() as f32 / 120.0;
            self.value = self.equation(place, first_item, second_item, coin_count, race_time);
        } else {
            self.value = self.settings.game.initial_price;
        }
    }

    fn equation(
        &self,
        place: f32,
        first_item: f32,
        second_item: f32,
        coin_count: f32,
        race_time: f32,
    ) -> i32 {
        // Items matter more the higher you're placed
        let item_scale = 0.5 + 0.5 * place;
        let w_items_eff = self.settings.game.items_weight * item_scale;

        // Renormalize so weights sum to 1.0
        let total = self.settings.game.place_weight + w_items_eff + self.settings.game.coins_weight;
        let w_place = self.settings.game.place_weight / total;
        let w_items = w_items_eff / total;
        let w_coins = self.settings.game.coins_weight / total;

        // Centered deviations from average
        let d_place = place - 0.5;
        let d_items = (first_item + second_item) / 2.0 - 0.5;
        let d_coins = coin_count - 0.5;

        // Weighted performance score
        let p = w_place * d_place + w_items * d_items + w_coins * d_coins;

        // Confidence factor scales with race progress
        let c = self.settings.game.confidence + (1.0 - self.settings.game.confidence) * race_time;

        // Final stock value
        let stock =
            self.settings.game.base_price * (1.0 + 2.0 * self.settings.game.volatility * c * p);
        0.max(stock.ceil() as i32)
    }

    pub fn sell_all_price(&self) -> i32 {
        0.max(
            (self.settings.game.base_price
                * ((24 - self.place) as f32 / self.settings.game.place_target as f32))
                .ceil() as i32,
        )
    }
}
impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}> [{}, {}¢, {}, {}] -> {}",
            self.racing, self.place, self.coin_count, self.first_item, self.second_item, self.value
        )
    }
}

pub fn string_to_number(input: &str) -> Option<u32> {
    let filtered: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    let parsed = filtered.parse::<u32>();
    if let Ok(number) = parsed {
        return Some(number);
    }
    None
}
