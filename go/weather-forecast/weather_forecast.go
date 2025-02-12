// Package weather returns the a string with the condition of the weather forecast for a given city.
package weather

var (
	// CurrentCondition is the condition of the weather for a given city.
	CurrentCondition string
	// CurrentLocation is the city for the weather forecast.
	CurrentLocation string
)

// Forecast function gets a city and a condition and returns a string describing the weather forecast.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
