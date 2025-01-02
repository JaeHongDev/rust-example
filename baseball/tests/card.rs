#[cfg(test)]
mod card_test {
    use baseball::domain::card::Card;

    #[test]
    #[should_panic]
    fn 카드의_pos는_0에서_3사이의_숫자_이하로_떨어질_수_없습니다() {
        (-2..=1).for_each(|x| {
            Card::new(x, 1);
        })
    }

    #[test]
    #[should_panic]
    fn 카드의_value는_1에서_9사이의_숫자로_구성해야합니다() {
        (-10..=0).chain(10..=15).for_each(|x| {
            Card::new(1, x);
        })
    }
}
