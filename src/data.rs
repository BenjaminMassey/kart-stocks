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

pub fn option_str_to_option_u32(input: Option<String>) -> Option<u32> {
    if let Some(str) = input {
        let input = str.parse::<u32>();
        if let Ok(int) = input {
            return Some(int);
        }
    }
    None
}