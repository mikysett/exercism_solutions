package hamming

import (
	"fmt"
)

func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, fmt.Errorf("Invalid arguments: can't calculate hamming distance of strings with different length")
	}

	hammingDistance := 0
	for i := range a {
		if a[i] != b[i] {
			hammingDistance++
		}
	}
	return hammingDistance, nil
}
