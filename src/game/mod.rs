use std::io::{self, Write};
mod state;
pub use self::state::State;
use rand::ThreadRng;

use parser;
use commands::Command::*;
use cards::{Card, Deck};

#[derive(PartialEq, Eq)]
enum Flow {
    Win, Loss, Continue, GameOver,
}

/// Plays the game
pub fn play(state: &mut State, rng: &mut ThreadRng) {
    let mut guard = true;
    println!("Type \"help\" for a list of commands.");
    print_status(state);
    while guard {
        let input = prompt("> ").expect("std::io failed.");
        if let Some(result) = parser::parse_input(&input) {
            let flow = match result {
                Bid(bid)   => handle_bid(state, bid),
                Hit(id)    => handle_hit(state, id),
                Stand      => handle_stand(state),
                Split      => handle_split(state),
                DoubleDown => handle_double(state),
                Help       => print_help(),
                Quit       => return,
            };

            guard = handle_winning(state, flow, rng);
        }

        if state.player_hands.is_empty() && state.current_bid > 0 {
            let card1 = state.deck.deal_card().unwrap(); // beginning of the game, it's safe to unwrap
            let card2 = state.deck.deal_card().unwrap();
            state.player_hands.push(vec![card1, card2]);
        }

        if state.dealer_hand.is_empty() && state.current_bid > 0 {
            let card = state.deck.deal_card().unwrap();
            state.dealer_hand.push(card);
        }

        print_status(state);
    }
}

// Helper functions

/// Prints the input, then asks for input from the console.
fn prompt(input: &str) -> io::Result<String> {
    let mut output = String::new();
    print!("{}", input);
    io::stdout().flush()?;
    io::stdin().read_line(&mut output)?;
    Ok(output)
}

/// Prints the available commands
fn print_help() -> Flow {
    println!("Available commands:");
    println!("<> = value, [] = optional value");
    println!("* \"bid <amount>\" -- bid the given amount.");
    println!("* \"split\" -- split the cards.");
    println!("* \"hit [index]\" -- let the dealer deal another card.");
    println!("* \"help\" -- print this help.");
    println!("* \"quit\" -- quit the game.");
    Flow::Continue
}

/// Prints the current user and dealer hands 
/// as well as the pot and the player's balance
fn print_status(state: &State) {
    println!("Balance: {}", state.earnings);
    println!("Pot: {}", state.current_bid);

    // if the dealer's hand isn't empty
    if !state.dealer_hand.is_empty() {
        print!("Dealer: ");
        for card in state.dealer_hand.iter() {
            print!("{} ", card);
        }
        println!("");
    }

    // if the player's hands aren't empty
    if !state.player_hands.is_empty() {
        // handle each hand separately
        for (i, hand) in state.player_hands.iter().enumerate() {
            print!("Hand {}: ", i + 1);
            for card in hand.iter() {
                print!("{} ", card);
            }

            let (a, b) = hand_total(&hand);
            
            if a == b {
                println!("Total: {}", a);
            } else {
                println!("Total: {} or {}", a, b);
            }
        }
    }
}

// Gets the running total of an entire hand worth of cards
fn hand_total(hand: &[Card]) -> (u32, u32) {
    hand.iter()
        .map(|card| card.get_value())
        .fold((0, 0), |acc, tpl| (acc.0 + tpl.0, acc.1 + tpl.1))
}

/// Handles bidding.
/// 
/// If the given amount is larger than the player's earnings, do nothing.
fn handle_bid(state: &mut State, amount: u64) -> Flow {
    if state.earnings - amount > 0 {
        state.earnings -= amount;
        state.current_bid += amount;
    }

    Flow::Continue
}

/// Handles standing a bid
/// 
/// Stop bidding and have the dealer play their hand.
fn handle_stand(state: &mut State) -> Flow {
    let totals = state.player_hands.iter().map(|hand| hand_total(&hand));
    loop {
        let dealer_total = hand_total(&state.dealer_hand);
        if dealer_total.0 >= 17 { break; }

        if let Some(card) = state.deck.deal_card() {
            state.dealer_hand.push(card);
        } else {
            println!("Out of cards.");
            return Flow::Loss;
        }
    }

    // TODO: handle winnings
    panic!("Stand unhandled");

    Flow::Continue
}

/// Hits the given card index, adding another card to it.
/// If the index is invalid, print an error and do nothing.
fn handle_hit(state: &mut State, id: usize) -> Flow {
    if state.player_hands.iter().all(|hand| hand.is_empty()) {
        println!("You don't have a hand, please bid first");
        return Flow::Continue;
    }

    if id < 1 {
        println!("Can't hit a hand less than 1");
        return Flow::Continue;
    }

    if let Some(card) = state.deck.deal_card() {
        state.player_hands[id - 1].push(card);
    } else {
        println!("No more cards left in deck");
        return Flow::Continue;
    }

    if state.player_hands.iter()
        .map(|hand| hand_total(&hand))
        .any(|tpl| tpl.0 <= 21) {
            Flow::Continue
        } else {
            Flow::Loss
        }
}

// split the hand in two
fn handle_split(state: &mut State) -> Flow {
    println!("Split unhandled");
    Flow::Continue
}

fn handle_double(state: &mut State) -> Flow {
    println!("Double down unhandled");
    Flow::Continue
}

fn handle_winning(state: &mut State, flow: Flow, rng: &mut ThreadRng) -> bool {
    let pot = state.current_bid;

    match flow {
        Flow::Win => {
            println!("You win the hand!");
            state.earnings += pot * 2;
            state.current_bid = 0;
            state.deck = Deck::new().shuffled(rng);
            true
        }
        Flow::Loss => {
            println!("You lose the hand.");
            // do nothing with the pot, the dealer is assumed to have infinite money
            state.current_bid = 0;
            state.deck = Deck::new().shuffled(rng);
            true
        }
        Flow::GameOver => {
            println!("Game Over.");
            false
        }
        Flow::Continue => {
            true
        }
    }
}