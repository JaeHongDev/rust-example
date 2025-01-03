pub struct Score {
    ball: i32,
    strike: i32,
}

impl Score {
    pub fn zero() -> Self {
        Self { ball: 0, strike: 0 }
    }

    pub fn strike(&self) -> Self {
        Self {
            ball: self.ball,
            strike: self.strike,
        }
    }

    pub fn ball(&self) -> Self {
        Self {
            ball: self.ball + 1,
            strike: self.strike,
        }
    }

    pub fn sum(&self, other: Score) -> Score {
        Self {
            ball: self.ball + other.ball,
            strike: self.strike + other.strike,
        }
    }

    pub fn is_three_strike(&self) -> bool {
        self.strike == 3
    }
}
