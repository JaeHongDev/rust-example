use std::ops::Not;

pub struct Card {
    pos: i8,
    value: i8,
}

impl Card {
    pub fn new(pos: i8, value: i8) -> Self {
        if pos < 0 {
            panic!("card의 pos는 0 밑의 값을 허용하지 않습니다.");
        }

        if (1..=9).contains(&value).not() {
            panic!("card의 값은 1~9사이의 숫자로 구성해야 합니다.")
        }

        Self { pos, value }
    }
}
