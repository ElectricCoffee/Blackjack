#[derive(Debug)]
pub enum Command {
    Bid(u64, usize),
    Hit(usize),
    Split(usize),
    Stand,
    DoubleDown,
    Quit,
    Help,
}