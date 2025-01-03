use std::ops::Not;

#[derive(Debug)]
pub struct Card {
    pos: i8,
    value: i32,
}

impl Card {
    pub fn new(pos: i8, value: i32) -> Self {
        Self::validate_card_pos(pos);
        Self::validate_card_value(value);

        Self { pos, value }
    }

    fn validate_card_pos(pos: i8) {
        pos.lt(&0)
            .then(|| panic!("card의 pos는 0 밑의 값을 허용하지 않습니다."));
    }

    fn validate_card_value(value: i32) {
        if (1..=9).contains(&value).not() {
            panic!("card의 값은 1~9사이의 숫자로 구성해야 합니다.");
        }
    }

    pub fn compare(&self, other: Card) -> Count {
        match (self.value == other.value, self.value == other.value) {
            (true, true) => Count::STRIKE,
            (true, false) => Count::BALL,
            (_) => Count::BALL,
        }
    }
}

pub enum Count {
    STRIKE,
    BALL,
}
