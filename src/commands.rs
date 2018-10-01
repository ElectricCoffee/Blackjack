#[derive(Debug)]
pub enum Command {
    Bid(u64, usize),
    Hit(usize),
    Split,
    Stand,
    DoubleDown,
    Quit,
    Help,
}