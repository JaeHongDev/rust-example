#[cfg(test)]

mod car_test {

    use racingcar::domain::car::Car;

    #[test]
    #[should_panic]
    fn 자동차의_이름은_5글자_미만입니다() {
        (5..=10)
            .map(|len| "x".repeat(len))
            .for_each(|name| { Car::new(name); });
    }

    #[test]
    #[should_panic]
    fn 자동차의_이름은_최소_1글자_이상입니다(){
        ["", " ", " "]
            .iter()
            .for_each(|name| {
                Car::new(name.parse().unwrap());
            });
    }

    #[test]
    fn 전진한다(){
        (4..=9).into_iter()
            .for_each(|condition_value| {
                let mut car = Car::new(String::from("name"));
                car.do_move(condition_value);
                assert_eq!(&1, car.get_pos());
            })
    }

    #[test]
    fn 정지한다(){
        (0..=3).into_iter()
            .for_each(|conditon_value| {
                let mut car = Car::new(String::from("name"));
                car.do_move(conditon_value);
                assert_eq!(car.get_pos(), &0);
            })
    }
}


#[test]
fn 자동차의_이름이_유효하지_않은_경우_오류가_발생합니다(){

}