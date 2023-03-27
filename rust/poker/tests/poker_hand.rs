use poker::card::card_ranks::CardRanks;
use poker::card::card_suits::CardSuits;
use poker::card::Card;
use poker::poker_hand::*;

#[test]
fn test_poker_hand() {
    assert_eq!(
        PokerHand::try_from("3S 4S 5D 6H JH"),
        Ok(PokerHand {
            cards: [
                (
                    1,
                    Card {
                        rank: CardRanks::new(3),
                        suit: CardSuits::Spades,
                    }
                ),
                (
                    1,
                    Card {
                        rank: CardRanks::new(4),
                        suit: CardSuits::Spades,
                    }
                ),
                (
                    1,
                    Card {
                        rank: CardRanks::new(5),
                        suit: CardSuits::Diamonds,
                    }
                ),
                (
                    1,
                    Card {
                        rank: CardRanks::new(6),
                        suit: CardSuits::Hearts,
                    }
                ),
                (
                    1,
                    Card {
                        rank: CardRanks::new(11),
                        suit: CardSuits::Hearts,
                    }
                ),
            ]
            .to_vec(),
            str_ref: &"3S 4S 5D 6H JH",
            score: Score::HighCard,
        })
    )
}

#[test]
fn test_hand_scores() {
    let straight_flush = PokerHand::try_from("JD 10D 9D 8D 7D").unwrap();
    assert_eq!(straight_flush.score, Score::StraightFlush);

    let four_of_a_kind = PokerHand::try_from("7D AS AD AC AH").unwrap();
    assert_eq!(four_of_a_kind.score, Score::FourOfAKind);

    let full_house = PokerHand::try_from("7D 7H AD AC AH").unwrap();
    assert_eq!(full_house.score, Score::FullHouse);

    let flush = PokerHand::try_from("AH JH 9H 8H 7H").unwrap();
    assert_eq!(flush.score, Score::Flush);

    let straight = PokerHand::try_from("10H 9S 8D 7D 6H").unwrap();
    assert_eq!(straight.score, Score::Straight);

    let three_of_a_kind = PokerHand::try_from("AH AD AS 2H 3D").unwrap();
    assert_eq!(three_of_a_kind.score, Score::ThreeOfAKind);

    let two_pairs = PokerHand::try_from("7H 7D 6H 5D 5S").unwrap();
    assert_eq!(two_pairs.score, Score::TwoPair);

    let one_pair = PokerHand::try_from("KH QH 10D 2D 2S").unwrap();
    assert_eq!(one_pair.score, Score::OnePair);

    let one_pair = PokerHand::try_from("KH QH 10D AC 2S").unwrap();
    assert_eq!(one_pair.score, Score::HighCard);
}
