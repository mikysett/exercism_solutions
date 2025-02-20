package booking

import (
	"fmt"
	"time"
)

// Schedule returns a time.Time from a string containing a date.
func Schedule(date string) time.Time {
	convertedDate, _ := time.Parse("1/2/2006 15:04:05", date)
	return convertedDate
}

// HasPassed returns whether a date has passed.
func HasPassed(date string) bool {
	convertedDate, _ := time.Parse("January 2, 2006 15:04:05", date)
	return time.Now().After(convertedDate)
}

// IsAfternoonAppointment returns whether a time is in the afternoon.
func IsAfternoonAppointment(date string) bool {
	convertedDate, _ := time.Parse("Monday, January 2, 2006 15:04:05", date)
	hour := convertedDate.Hour()
	return hour >= 12 && hour < 18
}

// Description returns a formatted string of the appointment time.
func Description(date string) string {
	convertedDate := Schedule(date)
	return fmt.Sprintf("You have an appointment on %s, %s %d, %d, at %02d:%02d.",
		convertedDate.Weekday(),
		convertedDate.Month().String(),
		convertedDate.Day(),
		convertedDate.Year(),
		convertedDate.Hour(),
		convertedDate.Minute(),
	)
}

// AnniversaryDate returns a Time with this year's anniversary.
func AnniversaryDate() time.Time {
	return time.Date(time.Now().Year(), 9, 15, 0, 0, 0, 0, time.UTC)
}
