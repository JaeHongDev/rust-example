#[cfg(test)]
mod player_test {
    use baseball::domain::card::Card;
    use baseball::domain::player::Player;

    #[test]
    #[should_panic]
    fn 플레이어는_3자리_카드를_가져야합니다() {
        Player::new(Vec::from([
            Card::new(0, 1),
            Card::new(1, 2),
            Card::new(2, 3),
            Card::new(2, 4),
        ]));
    }
}
