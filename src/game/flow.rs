#[derive(Debug, PartialEq, Eq)]
pub enum Flow {
    Continue, GameOver,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Winning {
    Win, Loss, Playing,
}