use crate::view::game_view::{GameView, Retry};
use baseball::domain::card::Card;
use baseball::domain::computer::Computer;
use baseball::domain::player::Player;

use crate::domain::card::Count;
use baseball::domain::score::Score;
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

    pub fn generate_random_number(&self) -> (Computer, Player) {
        let computer = self.generate_computer();
        let player = Player::from(self.view.get_player_number());

        println!("{:?}", computer);
        println!("{:?}", player);

        (computer, player)
    }

    pub fn referee(&self, player: Player, computer: Computer) -> Score {
        player.cards.iter().fold(Score::zero(), |score, card| {
            score.sum(
                computer
                    .cards
                    .iter()
                    .fold(Score::zero(), |score, &other_card| {
                        match (card.compare(other_card)) {
                            Count::STRIKE => score.strike(),
                            Count::BALL => score.ball(),
                        }
                    }),
            )
        })
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
