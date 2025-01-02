use std::ops::Not;

pub struct Card {
    pos: i8,
    value: i8,
}

impl Card {
    pub fn new(pos: i8, value: i8) -> Self {
        Self::validate_card_pos(pos);
        Self::validate_card_value(value);

        Self { pos, value }
    }

    fn validate_card_pos(pos: i8) {
        pos.eq(&0)
            .then(|| panic!("card의 pos는 0 밑의 값을 허용하지 않습니다."));
    }

    fn validate_card_value(value: i8) {
        if (1..=9).contains(&value).not() {
            panic!("card의 값은 1~9사이의 숫자로 구성해야 합니다.");
        }
    }
}
