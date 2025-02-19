package raindrops

import (
	"strconv"
	"strings"
)

type RainSound struct {
	number int
	sound  string
}

var rainSounds = []RainSound{
	{number: 3, sound: "Pling"},
	{number: 5, sound: "Plang"},
	{number: 7, sound: "Plong"},
}

func Convert(number int) string {
	var simphonyBuilder strings.Builder
	for _, sound := range rainSounds {
		if number%sound.number == 0 {
			simphonyBuilder.WriteString(sound.sound)
		}
	}

	simphony := simphonyBuilder.String()
	if len(simphony) == 0 {
		return strconv.Itoa(number)
	}
	return simphony
}
