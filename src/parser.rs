use regex::{self, Regex};
use commands::Command::{self, *};
use std::str::FromStr;

// all the regular expressions used in the program
lazy_static! {
    static ref BID: Regex    = Regex::new(r"(?i)bid ([0-9]+)").unwrap();
    static ref SPLIT: Regex  = Regex::new(r"(?i)split").unwrap();
    static ref HIT: Regex    = Regex::new(r"(?i)hit( [0-9]*)?").unwrap();
    static ref QUIT: Regex   = Regex::new(r"(?i)quit").unwrap();
    static ref HELP: Regex   = Regex::new(r"(?i)help").unwrap();
    static ref DOUBLE: Regex = Regex::new(r"(?i)double down").unwrap();
}

/// Parses a regex match and returns a command
/// 
/// Turns a match into a string, trims it, then parses it
fn parse_match<T, F>(mat: regex::Match, f: F) -> Option<Command> 
where T: FromStr, F: FnOnce(T) -> Command {
    mat.as_str().trim().parse::<T>().ok().map(f)
}

/// Parses a string using regular expressions, then throws back a Command
pub fn parse_input(input: &str) -> Option<Command> {
    if BID.is_match(input) {
        BID.captures(input)
            .and_then(|cap| cap.get(1))
            .and_then(|mat| parse_match(mat, Bid))
    } else if HIT.is_match(input) {
        HIT.captures(input)
            .and_then(|cap| cap.get(1))
            .and_then(|mat| parse_match(mat, Hit))
            .or(Some(Hit(1)))
    } else if SPLIT.is_match(input) {
        Some(Split)
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