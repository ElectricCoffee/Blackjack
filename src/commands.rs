#[derive(Debug)]
pub enum Command {
    Bid(i64, usize),
    Hit(usize),
    Split(usize),
    Stand,
    DoubleDown,
    Quit,
    Help,
}