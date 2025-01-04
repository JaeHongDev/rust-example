use crate::domain::score::Score;
use crate::view::game_view::Retry::{NO, YES};
use crate::view::input_view::Input;
use crate::view::output_view::Output;

pub struct GameView {
    input: Box<dyn Input>,
    output: Box<dyn Output>,
}

// constructor
impl GameView {
    pub fn new(input: Box<dyn Input>, output: Box<dyn Output>) -> Self {
        Self { input, output }
    }
}

// read game view
impl GameView {
    pub fn get_retry_number(&self) -> Retry {
        Retry::from(self.input.read_number(Option::from(
            "게임을 새로 시작하려면 1, 종료하려면 2를 입력하세요.",
        )))
    }

    pub fn get_player_number(&self) -> String {
        self.input.read_line(Option::from("숫자를 입력해주세요"))
    }
}

// print
impl GameView {
    pub fn start(&self) {
        self.output.println("숫자 야구게임을 시작합니다");
    }
    pub fn print_score(&self, score: &Score) {
        println!("{:?}", score);
        let msg = match (score.strike, score.ball) {
            (0, 0) => String::from("낫싱"),
            (_, 0) => format!("{} 스트라이크", score.strike),
            (0, _) => format!("{} 볼", score.ball),
            _ => format!("{} 스트라이크 {}볼", score.strike, score.ball)
        };
        println!("{}", msg);
    }
    pub fn print_game_start_message(&self) {
        println!("숫자 야구게임을 시작합니다.");
    }
}

impl GameView {}

pub enum Retry {
    YES,
    NO,
}

impl Retry {
    fn from(n: i64) -> Retry {
        match n {
            1 => YES,
            2 => NO,
            _ => panic!("관련 없는 숫자"),
        }
    }
}
