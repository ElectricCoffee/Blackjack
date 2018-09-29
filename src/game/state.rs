use cards::*;
use rand::ThreadRng;

pub struct State {
    pub deck: Deck,
    pub player_hands: Vec<Vec<Card>>,
    pub dealer_hand: Vec<Card>,
    pub earnings: u64,
    pub current_bid: u64,
}

impl State {
    pub fn new(money: u64, rng: &mut ThreadRng) -> State {
        State { 
            deck: Deck::new().shuffled(rng),
            player_hands: Vec::new(),
            dealer_hand: Vec::new(),
            earnings: money, 
            current_bid: 0,
        }
    }
}