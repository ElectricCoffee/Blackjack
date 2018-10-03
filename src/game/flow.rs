#[derive(PartialEq, Eq)]
pub enum Flow {
    Continue, GameOver,
}

#[derive(PartialEq, Eq)]
pub enum Winning {
    Win, Loss, Playing,
}