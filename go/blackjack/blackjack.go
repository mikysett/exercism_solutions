package blackjack

var cardValues = map[string]int{
	"ace":   11,
	"eight": 8,
	"two":   2,
	"nine":  9,
	"three": 3,
	"ten":   10,
	"four":  4,
	"jack":  10,
	"five":  5,
	"queen": 10,
	"six":   6,
	"king":  10,
	"seven": 7,
	"other": 0,
}

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	return cardValues[card]
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	c1, c2, dc := ParseCard(card1), ParseCard(card2), ParseCard(dealerCard)
	switch {
	case c1 == 11 && c2 == 11:
		return "P"
	case c1+c2 == 21 && dc < 10:
		return "W"
	case c1+c2 >= 17:
		return "S"
	case c1+c2 <= 11 || dc >= 7:
		return "H"
	default:
		return "S"
	}
}
