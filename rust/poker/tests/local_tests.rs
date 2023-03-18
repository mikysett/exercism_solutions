use poker::PokerHand;
use poker::Score;
use poker::card::Card;
use poker::card::card_ranks::CardRanks;
use poker::card::card_suits::CardSuits;

#[test]
fn test_poker_hand() {
    assert_eq!(
        PokerHand::try_from("3S 4S 5D 6H JH"),
        Ok(PokerHand {
            cards: [
                Card {rank: CardRanks::N3,suit: CardSuits::Spades,},
                Card {rank: CardRanks::N4,suit: CardSuits::Spades,},
                Card {rank: CardRanks::N5,suit: CardSuits::Diamonds,},
                Card {rank: CardRanks::N6,suit: CardSuits::Hearts,},
                Card {rank: CardRanks::J,suit: CardSuits::Hearts,},
            ].to_vec(),
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
