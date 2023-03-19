use std::cmp::Ordering;

use crate::card::*;
use crate::poker_hand::PokerHand;

pub mod card;
pub mod poker_hand;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let poker_hands: Result<Vec<PokerHand>, <Card as TryFrom<&str>>::Error> = hands
        .iter()
        .map(|hand| PokerHand::try_from(*hand))
        .collect();

    let mut result = vec![];
    match poker_hands {
        Ok(mut my_hands) => {
            my_hands.sort_by(|a, b| b.partial_cmp(a).unwrap());

            for hand in &my_hands {
                if hand.partial_cmp(&my_hands[0]).unwrap() == Ordering::Equal {
                    result.push(hand.str_ref);
                } else {
                    break;
                }
            }
        }
        Err(_) => {
            panic!("Bad input");
        }
    }
    return result;
}
