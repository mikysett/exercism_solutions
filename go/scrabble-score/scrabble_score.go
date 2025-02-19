package scrabble

import "strings"

var letterScores = map[byte]int{
	// Group with score: 1
	'A': 1, 'E': 1, 'I': 1, 'O': 1, 'U': 1, 'L': 1, 'N': 1, 'R': 1, 'S': 1, 'T': 1,
	// Group with score: 2
	'D': 2, 'G': 2,
	// Group with score: 3
	'B': 3, 'C': 3, 'M': 3, 'P': 3,
	// Group with score: 4
	'F': 4, 'H': 4, 'V': 4, 'W': 4, 'Y': 4,
	// Group with score: 5
	'K': 5,
	// Group with score: 8
	'J': 8, 'X': 8,
	// Group with score: 10
	'Q': 10, 'Z': 10,
}

func Score(word string) int {
	totalScore := 0
	word = strings.ToUpper(word)

	for i := range word {
		totalScore += letterScores[word[i]]
	}
	return totalScore
}
