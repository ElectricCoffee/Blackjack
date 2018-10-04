use rand::{ThreadRng, Rng};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Suit {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}

/// The 13 possible values of a card
const VALUES: [char; 13] =
    ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'X', 'J', 'Q', 'K'];

/// The four suits of a card
const SUITS: [Suit; 4] = [Suit::Clubs, Suit::Hearts, Suit::Spades, Suit::Diamonds];

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Card {
    suit: Suit,
    value: char,
}

impl Card {
    /// Returns the possible values each value represents.
    /// In particular Ace is 1 and 11, the rest are just duplicates of a single one.
    pub fn get_value(&self) -> (u32, u32) {
        match self.value {
            '2' => (2, 2),
            '3' => (3, 3),
            '4' => (4, 4),
            '5' => (5, 5),
            '6' => (6, 6),
            '7' => (7, 7),
            '8' => (8, 8),
            '9' => (9, 9),
            'X' | 'J' | 'Q' | 'K' => (10, 10),
            'A' => (1, 11),
            _ => panic!("Unknown card value"), 
        }
    }

    pub fn symbolic_value(&self) -> char {
        self.value
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Suit::*;

        let suit = match self.suit {
            Clubs    => "♣️",
            Hearts   => "♥️",
            Spades   => "♠️",
            Diamonds => "♦️",
        };

        let value = if self.value == 'X' { "10".into() } else { self.value.to_string() };

        write!(f, "{} {}", suit, value)
    }
}

/// Wrapper for a Vec of cards
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut result = Vec::new();
        for suit in SUITS.iter().cloned() {
            for value in VALUES.iter().cloned() {
                result.push(Card { suit, value })
            }
        }

        Deck { cards: result }
    }

    /// Shuffles the deck mutably
    pub fn shuffle(&mut self, rng: &mut ThreadRng) {
        rng.shuffle(&mut self.cards);
    }

    /// Consumes the deck, shuffles it, then returns a new deck
    pub fn shuffled(mut self, rng: &mut ThreadRng) -> Deck {
        self.shuffle(rng);
        self
    }

    /// Pops a card from the deck
    pub fn deal_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}