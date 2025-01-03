use crate::domain::card::Card;

#[derive(Debug)]
pub struct Computer {
    pub cards: Vec<Card>,
}

impl Computer {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }
}
