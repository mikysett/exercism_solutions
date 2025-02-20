package collatzconjecture

import (
	"fmt"
)

func CollatzConjecture(n int) (int, error) {
	if n < 1 {
		return 0, fmt.Errorf("Invalid argument: must be a positive integer > 0.")
	}
	return solveCollatz(n), nil
}

func solveCollatz(n int) int {
	if n == 1 {
		return 0
	}
	if n%2 == 0 {
		return 1 + solveCollatz(n/2)
	}
	return 1 + solveCollatz(n*3+1)
}
