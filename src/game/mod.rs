mod state;
mod flow;

pub use self::state::State;
pub use self::flow::{Winning, Flow};

use std::io::{self, Write};
use rand::ThreadRng;

use parser;
use commands::Command::*;
use cards::{Card, Deck};

/// Plays the game
pub fn play(state: &mut State, rng: &mut ThreadRng) {
    let mut guard = Flow::Continue;
    println!("Type \"help\" for a list of commands.");
    print_status(state);
    while guard == Flow::Continue {
        let input = prompt("> ").expect("std::io failed.");
        if let Some(result) = parser::parse_input(&input) {
            match result {
                Bid(bid, id) => handle_bid(state, bid, id - 1),
                Hit(id)      => handle_hit(state, id - 1),
                Split(id)    => handle_split(state, id - 1),
                Stand        => handle_stand(state),
                DoubleDown   => handle_double(state),
                Help         => print_help(),
                Quit         => return,
            }

            guard = handle_winning(state, rng);
        }

        if state.player_hands.is_empty() && state.current_bids.iter().any(|&bid| bid > 0) {
            let card1 = state.deck.deal_card().unwrap(); // beginning of the game, it's safe to unwrap
            let card2 = state.deck.deal_card().unwrap();
            state.player_hands.push(vec![card1, card2]);
        }

        if state.dealer_hand.is_empty() && state.current_bids.iter().any(|&bid| bid > 0) {
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
fn print_help() {
    println!("Available commands:");
    println!("<> = value, [] = optional value");
    println!("* \"bid <amount> [index]\" -- bid the given amount.");
    println!("* \"split [index]\" -- split the cards.");
    println!("* \"hit [index]\" -- let the dealer deal another card.");
    println!("* \"help\" -- print this help.");
    println!("* \"quit\" -- quit the game.");
}

/// Prints the current user and dealer hands 
/// as well as the pot and the player's balance
fn print_status(state: &State) {
    println!("Balance: {}", state.earnings);

    if !state.current_bids.is_empty() {
        print!("Pot: ");
        for bid in state.current_bids {
            println!("{} ", bid);
        }
        println!("");
    }

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
fn handle_bid(state: &mut State, amount: u64, hand_index: usize) {
    if hand_index > state.current_bids.len() {
        println!("You cannot bid to a hand you do not possess.");
    }
    // TODO: deal with per-hand earnings
    if state.earnings - amount >= 0 {
        state.earnings -= amount;
        state.current_bids[hand_index] += amount;
    } else {
        println!("You can't bid more than you have.");
    }
}

/// Handles standing a bid
/// 
/// Stop bidding and have the dealer play their hand.
fn handle_stand(state: &mut State) {
    state.player_wins = Vec::with_capacity(state.player_hands.len()); // ensure it's zeroed out
    let totals = state.player_hands.iter().map(|hand| hand_total(&hand));
    let mut dealer_total = (0, 0);
    loop {
        dealer_total = hand_total(&state.dealer_hand);
        if dealer_total.0 >= 17 { break; }

        if let Some(card) = state.deck.deal_card() {
            state.dealer_hand.push(card);
        } else {
            println!("Out of cards.");
        }
    }

    for (i, total) in totals.enumerate() {
        state.player_wins[i] = if total.0 > 21 {
            Winning::Loss
        } else if total.0 > dealer_total.0 || total.1 > dealer_total.1 {
            Winning::Win
        } else {
            Winning::Loss
        };
    }
}

/// Hits the given card index, adding another card to it.
/// If the index is invalid, print an error and do nothing.
fn handle_hit(state: &mut State, id: usize) {
    if state.player_hands.iter().all(|hand| hand.is_empty()) {
        println!("You don't have a hand, please bid first");
    }

    if id < 1 {
        println!("Can't hit a hand less than 1");
    }

    if let Some(card) = state.deck.deal_card() {
        state.player_hands[id].push(card);
    } else {
        println!("No more cards left in deck");
    }
}

// split the hand in two
fn handle_split(state: &mut State, idx: usize) {
    let first_bid = state.current_bids[idx];
    let mut old_hand = state.player_hands[idx];

    // ensure enough money
    if state.earnings - first_bid < 0 {
        println!("You cannot split the hand, your pot is too small.");
        return;
    }

    // ensure cards match
    if old_hand[0].symbolic_value() != old_hand[1].symbolic_value() {
        println!("Can't split a hand with different card values.");
        return;
    }

    // ensure hand size
    if old_hand.len() != 2 {
        println!("Can't split a hand of size {}.", old_hand.len());
        return;
    }

    // we already know the hand is large enough, so it's safe to pop
    let card = old_hand.pop().unwrap(); // move one of the cards to the new hand
    let mut new_hand = vec![card];

    // we just assume the deck has enough cards
    let card1 = state.deck.deal_card().expect("Ran out of cards in deck.");
    let card2 = state.deck.deal_card().expect("Ran out of cards in deck.");

    // add the cards from the deck to the hands
    old_hand.push(card1);
    new_hand.push(card2);

    // add the new hand to the vec of player hands
    state.player_hands.push(new_hand);

    // move a bit of the player's earnings into the new hand's pot.
    state.earnings -= first_bid;
    state.current_bids.push(first_bid);
}

fn handle_double(state: &mut State) {
    println!("Double down unhandled");
    Flow::Continue
}

fn handle_winning(state: &mut State, rng: &mut ThreadRng) -> Flow {
    // if any hand is still playing, just return
    if state.player_wins.iter().any(|&w| w == Winning::Playing) {
        return Flow::Continue;
    }

    let mut winnings = state.earnings;
    let pots = state.current_bids;

    for (i, winning) in state.player_wins.iter().enumerate() {
        match winning {
            Winning::Win => {
                println!("You win on hand {}!", i + 1);
                winnings += pots[i] * 2;
            }
            Winning::Loss => {
                println!("You lose on hand {}.", i + 1);
                // do nothing with the pot, the dealer is assumed to have infinite money
            }
        }
    }

    if winnings == 0 {
        return Flow::GameOver;
    }

    // reset the state with the new earnings
    *state = State::new(winnings, rng);

    Flow::Continue
}