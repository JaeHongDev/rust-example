use std::ops::Not;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_tuple((l1, l2): (i64, i64), (r1, r2): (i64, i64)) -> (i64, i64) {
    println!("{:?}, {:?}", (l1, l2), (r1, r2));

    (l1 + r1, l2 + r2)
}

pub fn str_to_baseball_num(input: &str) -> Vec<u32> {
    let vec: Vec<u32> = str::chars(input)
        .map(|c| c.to_digit(10))
        .map(|option_c| option_c.expect("문자열이 포함됐습니다"))
        .filter(|n| (1..=9).contains(n))
        .collect();

    vec.len().eq(&3).not().then(|| panic!("3자리가 아닙니다"));

    vec
}

pub mod score {
    #[derive(Debug)]
    pub struct Score {
        ball: i64,
        strike: i64,
    }

    impl Score {
        pub fn add_ball(&mut self) {
            self.ball += 1;
        }
        pub fn add_strike(&mut self) {
            self.strike += 1;
        }

        pub fn sum(&self, score: Score) -> Score {
            Score {
                ball: self.ball + score.ball,
                strike: self.strike + score.strike,
            }
        }

        pub fn strike(&self) -> Score {
            Score {
                ball: self.ball,
                strike: self.strike + 1,
            }
        }

        pub fn ball(&self) -> Score {
            Score {
                ball: self.ball + 1,
                strike: self.strike + 1,
            }
        }

        pub fn init() -> Self {
            Self { ball: 0, strike: 0 }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::score::Score;

    #[test]
    fn 학습_테스트1() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn 개별_문자열을_숫자로_바꾸기() {
        let msg = "a23";

        let vec: Vec<u32> = str::chars(msg)
            .map(|c| c.to_digit(10))
            .map(|option_c| option_c.expect("에러"))
            .collect();

        println!("{:?}", vec);
    }

    #[test]
    #[should_panic(expected = "3자리가 아닙니다")]
    fn 문자열은_3자리이며_1에서_9사이의_숫자임() {
        let input = "2124";

        str_to_baseball_num(input);
    }

    #[test]
    fn 백터를_비교함() {
        let v1: Vec<i64> = (1..=3).collect();
        let v2: Vec<i64> = (1..=3).collect();
        // let v2 : Vec<i64> = (3..=5).collect();

        println!("{:?}", v1);
        println!("{:?}", v2);

        let mut ball = 0;
        let mut strike = 0;

        for (v1_index, v1_item) in v1.iter().enumerate() {
            for (v2_index, v2_item) in v2.iter().enumerate() {
                match (v1_item == v2_item, v1_index == v2_index) {
                    (true, false) => ball += 1,
                    (true, true) => strike += 1,
                    _ => (),
                }
            }
        }

        println!("{} {}", strike, ball);
    }

    #[test]
    fn 함수형_스타일로_벡터를_비교함() {
        let v1: Vec<i64> = (1..=3).collect();
        let v2: Vec<i64> = (2..=4).collect();

        println!("{:?} {:?}", v1, v2);

        let r: (i64, i64) = v1.iter().enumerate().fold((0, 0), |acc, (index, item)| {
            add_tuple(
                acc,
                v2.iter()
                    .enumerate()
                    .fold((0, 0), |(strike, ball), (other_index, other_item)| {
                        match (item == other_item, index == other_index) {
                            (true, true) => (strike + 1, ball),
                            (true, false) => (strike, ball + 1),
                            _ => (strike, ball),
                        }
                    }),
            )
        });

        println!("{:?}", r);
    }

    #[test]
    fn 벡터_비교_구조체버전() {
        let v1: Vec<i64> = (1..=3).collect();
        let v2: Vec<i64> = (1..=3).collect();

        let mut score = Score::init();

        for (v1_index, v1_item) in v1.iter().enumerate() {
            for (v2_index, v2_item) in v2.iter().enumerate() {
                match (v1_item == v2_item, v1_index == v2_index) {
                    (true, true) => score.add_strike(),
                    (true, false) => score.add_ball(),
                    _ => (),
                }
            }
        }

        println!("{:?}", score);
    }

    #[test]
    fn 함수형_스타일_백터_비교_구조체_버전() {
        let v1: Vec<i64> = (1..=3).collect();
        let v2: Vec<i64> = (1..=3).collect();

        let score = v1
            .iter()
            .enumerate()
            .fold(Score::init(), |score, (index, item)| {
                score.sum(v2.iter().enumerate().fold(
                    Score::init(),
                    |score, (other_index, other_item)| match (
                        item == other_item,
                        index == other_index,
                    ) {
                        (true, true) => score.strike(),
                        (true, false) => score.ball(),
                        _ => score,
                    },
                ))
            });

        println!("{:?}", score)
    }
}
