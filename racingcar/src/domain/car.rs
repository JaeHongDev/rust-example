use std::ops::{Not, RangeInclusive};

pub struct Car {
    name: String,
    pos: i32
}

impl Car {

    const CAR_NAME_SIZE_RANGE: RangeInclusive<usize> = 1..=5;

    pub fn new(name: String) -> Self {
        Self::validate_name(&name);
        Self { name , pos: 0 }
    }

    pub fn do_move(&mut self, condition_value: usize) {
        if condition_value >= 4 {
            self.pos += 1;
        }
    }

     pub fn get_pos(&self ) -> &i32{
         &self.pos
    }

}

impl Car {
    fn validate_name(name: &String) {
        Self::CAR_NAME_SIZE_RANGE
            .contains(&name.trim().len())
            .not()
            .then(|| panic!("이름의 유효 길이는 1~5 사이입니다."));
    }
}