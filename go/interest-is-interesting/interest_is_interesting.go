package interest

// InterestRate returns the interest rate for the provided balance.
func InterestRate(balance float64) float32 {
	if balance >= 5000 {
		return 2.475
	} else if balance >= 1000 {
		return 1.621
	} else if balance >= 0 {
		return 0.5
	}
	return float32(3.213)
}

// Interest calculates the interest for the provided balance.
func Interest(balance float64) float64 {
	return balance / 100 * float64(InterestRate(balance))
}

// AnnualBalanceUpdate calculates the annual balance update, taking into account the interest rate.
func AnnualBalanceUpdate(balance float64) float64 {
	return balance + float64(Interest(balance))
}

// YearsBeforeDesiredBalance calculates the minimum number of years required to reach the desired balance.
func YearsBeforeDesiredBalance(balance, targetBalance float64) int {
	yearsCount := 0
	for updatedBalance := balance; updatedBalance < targetBalance; yearsCount++ {
		updatedBalance = AnnualBalanceUpdate(updatedBalance)
	}
	return yearsCount
}
