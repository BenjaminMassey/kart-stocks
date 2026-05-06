pub const PLACEMENT_SLOT: u64 = 0;
pub const ITEMS_SLOT: u64 = 1;

const PLACEMENT_COEFFICIENT: f32 = 1.0;
const ITEM_COEFFICIENT: f32 = 1.0;
const COIN_COEFFICIENT: f32 = 1.0;
const TOTAL_MULT: f32 = 15.0;
static ITEM_VALUES: phf::Map<&'static str, i32> = phf::phf_map! {
    "none" => 0,
    "blooper" => 1,
    "feather" => 2,
    "coin" => 5,
    "coin-block" => 5,
    "boomerang" => 5,
    "banana" => 15,
    "bombomb" => 15,
    "green-shell" => 15,
    "horn" => 20,
    "lightning" => 20,
    "red-shell" => 20,
    "double-banana" => 35,
    "double-green-shell" => 35,
    "double-red-shell" => 40,
    "mushroom" => 40,
    "triple-banana" => 45,
    "triple-green-shell" => 45,
    "triple-red-shell" => 50,
    "double-mushroom" => 70,
    "triple-mushroom" => 80,
    "boo" => 90,
    "golden-mushroom" => 90,
    "star" => 95,
    "mega-mushroom" => 100,
    "bullet-bill" => 100,
}; // should remain range from 0 through 100

pub fn valid_item(item: &str) -> bool {
    ITEM_VALUES.contains_key(item)
}

#[derive(Clone)]
pub struct State {
    pub running: bool,
    pub time: std::time::Instant,
    pub place: u32,
    pub first_item: String,
    pub second_item: String,
    pub coin_count: u32,
    pub value: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            running: true,
            time: std::time::Instant::now(),
            place: 24,
            first_item: "none".to_owned(),
            second_item: "none".to_owned(),
            coin_count: 0,
            value: 0,
        }
    }

    pub fn update_value(&mut self) {
        self.value = (((((24 - self.place) as f32 / 23.0) * PLACEMENT_COEFFICIENT)
            + ((ITEM_VALUES[&self.first_item] as f32 / 100.0) * ITEM_COEFFICIENT)
            + ((ITEM_VALUES[&self.second_item] as f32 / 100.0) * ITEM_COEFFICIENT)
            + (((20 - self.coin_count) as f32 / 19.0) * COIN_COEFFICIENT))
            * TOTAL_MULT)
            .ceil() as i32
    }
}
impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}¢, {}, {}] -> {}",
            self.place, self.coin_count, self.first_item, self.second_item, self.value
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
