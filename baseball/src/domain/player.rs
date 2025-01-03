use std::ops::Not;

use crate::domain::card;
use card::Card;

#[derive(Debug)]
pub struct Player {
    pub cards: Vec<Card>,
}

impl Player {
    pub fn new(cards: Vec<Card>) -> Self {
        Self::validate_card_length(&cards);
        Self { cards }
    }

    pub fn from(str: String) -> Self {
        let vec: Vec<Card> = str::chars(str.as_str())
            .map(|c| c.to_digit(10))
            .map(|option_c| option_c.expect("문자열이 포함됐습니다"))
            .filter(|n| (1..=9).contains(n))
            .enumerate()
            .map(|(idx, x)| Card::new(idx as i8, x as i32))
            .collect();

        vec.len().eq(&3).not().then(|| panic!("3자리가 아닙니다"));

        Self::new(vec)
    }

    fn validate_card_length(cards: &Vec<Card>) {
        cards
            .len()
            .eq(&3)
            .not()
            .then(|| panic!("사용자는 3자리 숫자를 가져야 합니다."));
    }
}
