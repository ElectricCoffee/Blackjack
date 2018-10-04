use cards::*;
use rand::ThreadRng;
use game::flow::Winning;

pub struct State {
    pub deck: Deck,
    pub player_hands: Vec<Vec<Card>>,
    pub player_wins: Vec<Winning>,
    pub dealer_hand: Vec<Card>,
    pub current_bids: Vec<i64>,
    pub earnings: i64,
}

impl State {
    pub fn new(money: i64, rng: &mut ThreadRng) -> State {
        State { 
            deck: Deck::new().shuffled(rng),
            player_hands: vec![],
            player_wins:  vec![Winning::Playing],
            dealer_hand:  vec![],
            current_bids: vec![0],
            earnings: money, 
        }
    }
}