use regex::{self, Regex};
use commands::Command::{self, *};
use std::str::FromStr;

// all the regular expressions used in the program
lazy_static! {
    static ref BID: Regex    = Regex::new(r"(?i)bid ([0-9]+)( [0-9]*)?").unwrap();
    static ref SPLIT: Regex  = Regex::new(r"(?i)split( [0-9]*)?").unwrap();
    static ref HIT: Regex    = Regex::new(r"(?i)hit( [0-9]*)?").unwrap();
    static ref STAND: Regex  = Regex::new(r"(?i)stand").unwrap();
    static ref QUIT: Regex   = Regex::new(r"(?i)quit").unwrap();
    static ref HELP: Regex   = Regex::new(r"(?i)help").unwrap();
    static ref DOUBLE: Regex = Regex::new(r"(?i)double down").unwrap();
}

/// Parses a regex match and returns a command
/// 
/// Turns a match into a string, trims it, then parses it
fn parse_match<T: FromStr>(mat: regex::Match) -> Option<T> {
    mat.as_str().trim().parse::<T>().ok()
}

/// Parses a string using regular expressions, then throws back a Command
pub fn parse_input(input: &str) -> Option<Command> {
    if BID.is_match(input) {
        BID.captures(input)
            .and_then(|cap| {
                let bid = cap.get(1).and_then(parse_match); // get 1st capture, and parse it to a u64
                let idx = cap.get(2).and_then(parse_match).or(Some(1)); // get 2nd capture, and parse it to a usize

                bid.and_then(|b| idx.map(|i| Bid(b, i))) // wrap it in a Bid
            })
    } else if HIT.is_match(input) {
        HIT.captures(input)
            .and_then(|cap| cap.get(1)) // get 1st capture
            .and_then(parse_match) // parse to a usize
            .map(Hit) // wrap it in a Hit
            .or(Some(Hit(1))) // if None, return Hit(1)
    } else if SPLIT.is_match(input) {
        SPLIT.captures(input)
            .and_then(|cap| cap.get(1))
            .and_then(parse_match)
            .map(Split)
            .or(Some(Split(1)))
    } else if STAND.is_match(input) {
        Some(Stand)
    } else if QUIT.is_match(input) {
        Some(Quit)
    } else if HELP.is_match(input) {
        Some(Help)
    } else if DOUBLE.is_match(input) {
        Some(DoubleDown)
    } else {
        None
    }
}