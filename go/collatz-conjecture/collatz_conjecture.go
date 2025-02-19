package collatzconjecture

import (
	"fmt"
)

func CollatzConjecture(n int) (int, error) {
	if n < 1 {
		return 0, fmt.Errorf("Invalid argument: must be a positive integer > 0.")
	}
	return solveCollatz(n, 0)
}

func solveCollatz(n int, stepsCount int) (int, error) {
	if n == 1 {
		return stepsCount, nil
	}
	if n%2 == 0 {
		return solveCollatz(n/2, stepsCount+1)
	}
	return solveCollatz(n*3+1, stepsCount+1)
}
