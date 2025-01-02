use crate::domain::card::Card;

pub struct Computer {
    cards: Vec<Card>,
}

impl Computer {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }
}
