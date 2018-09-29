#[derive(Debug)]
pub enum Command {
    Bid(u64),
    Hit(usize),
    Split,
    Stand,
    DoubleDown,
    Quit,
    Help,
}