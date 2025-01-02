#[cfg(test)]
mod card_test {
    use baseball::domain::card::Card;

    #[test]
    fn 카드_생성_테스트() {
        Card::new(1, 2);
    }
}
