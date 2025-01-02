use std::ops::Not;

use crate::domain::card;
use card::Card;

pub struct Player {
    cards: Vec<Card>,
}

impl Player {
    pub fn new(cards: Vec<Card>) -> Self {
        cards
            .len()
            .eq(&3)
            .not()
            .then(|| panic!("사용자는 3자리 숫자를 가져야 합니다."));

        Self { cards }
    }
}
