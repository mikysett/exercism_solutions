package lasagna

// TODO: define the 'PreparationTime()' function
func PreparationTime(layers []string, preparationTime int) int {
	if preparationTime == 0 {
		preparationTime = 2
	}
	return len(layers) * preparationTime
}

// TODO: define the 'Quantities()' function
func Quantities(layers []string) (noodlesGrams int, sauceLiters float64) {
	countNoodles := 0
	countSauce := 0
	for _, layer := range layers {
		if layer == "noodles" {
			countNoodles++
		} else if layer == "sauce" {
			countSauce++
		}
	}
	return countNoodles * 50, float64(countSauce) * 0.2
}

// TODO: define the 'AddSecretIngredient()' function
func AddSecretIngredient(friendsList, myList []string) {
	myList[len(myList)-1] = friendsList[len(friendsList)-1]
}

// TODO: define the 'ScaleRecipe()' function
func ScaleRecipe(quantities []float64, portions int) []float64 {
	scale := float64(portions) / 2
	scaledQuantities := make([]float64, 0, len(quantities))
	for _, quantity := range quantities {
		scaledQuantities = append(scaledQuantities, quantity*scale)
	}
	return scaledQuantities
}

// Your first steps could be to read through the tasks, and create
// these functions with their correct parameter lists and return types.
// The function body only needs to contain `panic("")`.
//
// This will make the tests compile, but they will fail.
// You can then implement the function logic one by one and see
// an increasing number of tests passing as you implement more
// functionality.
