use std::cmp::Ordering;

use crate::poker_hand::PokerHand;

pub mod card;
pub mod poker_hand;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut poker_hands: Vec<PokerHand> = hands
        .iter()
        .map(|hand| {
            PokerHand::try_from(*hand)
                .unwrap_or_else(|err| panic!("Error at line: {}\n{}", hand, err))
        })
        .collect();

    poker_hands.sort_by(|a, b| b.partial_cmp(a).unwrap());

    poker_hands
        .iter()
        .filter(|hand| hand.partial_cmp(&&poker_hands[0]).unwrap() == Ordering::Equal)
        .map(|hand| hand.str_ref)
        .collect::<Vec<&str>>()
}
