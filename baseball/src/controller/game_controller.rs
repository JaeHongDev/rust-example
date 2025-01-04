use std::ops::Not;
use crate::view::game_view::{GameView, Retry};
use crate::domain::score::Score;
use baseball::domain::card::{Card,Count};
use baseball::domain::computer::Computer;
use baseball::domain::player::Player;
use rand::prelude::SliceRandom;

pub struct GameController {
    view: GameView,
}

impl GameController {
    pub fn new(view: GameView) -> Self {
        Self { view }
    }

    pub fn game_init(&self) {
        self.view.print_game_start_message();
    }

    pub fn play(&self){

        let computer = self.generate_computer();

        while self.referee(&computer).is_three_strike().not() {}

        match self.request_retry() {
            Retry::YES => self.play(),
            Retry::NO => {}
        }


    }

    pub fn generate_random_number(&self) -> (Computer, Player) {
        let computer = self.generate_computer();
        let player = Player::from(self.view.get_player_number());


        (computer, player)
    }

    pub fn referee(&self, computer: &Computer) -> Score {
        let player = Player::from(self.view.get_player_number());
        println!("{:?}", computer);
        println!("{:?}", player);


        let score = player.cards.iter().fold(Score::zero(), |score, card| {
            score.sum(
                computer
                    .cards
                    .iter()
                    .fold(Score::zero(), |score, other_card| {
                        match (card.compare(&other_card)) {
                            Count::STRIKE => score.strike(),
                            Count::BALL => score.ball(),
                            Count::NO => score
                        }
                    }),
            )
        });

        self.view.print_score(&score);

        score
    }

    pub fn request_retry(&self) -> Retry {
        self.view.get_retry_number()
    }

    fn generate_computer(&self) -> Computer {
        let mut nums: Vec<i32> = (1..=9).map(|x| x).collect();

        nums.shuffle(&mut rand::rng());

        Computer::new(
            nums.iter()
                .take(3)
                .enumerate()
                .map(|(idx, &x)| Card::new(idx as i8, x))
                .collect::<Vec<Card>>(),
        )
    }
}
