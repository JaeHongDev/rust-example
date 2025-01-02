pub struct Card {
    pos: i8,
    value: i8,
}

impl Card {
    pub fn new(pos: i8, value: i8) -> Self {
        Self { pos, value }
    }
}
