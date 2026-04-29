#[derive(Debug)]
pub struct State {
    pub place: u32,
    pub first_item: Option<String>,
    pub second_item: Option<String>,
    pub coin_count: u32,
}
impl State {
    pub fn new() -> Self {
        Self {
            place: 24,
            first_item: None,
            second_item: None,
            coin_count: 0,
        }
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



