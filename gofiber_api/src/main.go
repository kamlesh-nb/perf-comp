package main

import (
	"math/rand"
	"time"

	"github.com/gofiber/fiber/v2"
)

var sumarries [10]string

type WeatherForecast struct {
	Date         time.Time
	TemperatureC int
	TemperatureF float64
	Summary      string
}

func getWeatherForecast() [5]WeatherForecast {

	var forecast [5]WeatherForecast

	for i := 1; i < 6; i++ {

		c := rand.Intn(55)
		f := float64(32) + (float64(c) / 0.5556)
		e := rand.Intn(9)

		forecast[i-1] = WeatherForecast{
			Date:         time.Now().AddDate(0, 0, 1),
			TemperatureC: c,
			TemperatureF: f,
			Summary:      sumarries[e],
		}

	}

	return forecast
}

func main() {

	sumarries = [...]string{
		"Freezing",
		"Bracing",
		"Chilly",
		"Cool",
		"Mild",
		"Warm",
		"Balmy",
		"Hot",
		"Sweltering",
		"Scorching",
	}

	app := fiber.New()

	app.Get("/weather", func(c *fiber.Ctx) error {
		return c.JSON(getWeatherForecast())
	})

	app.Listen("0.0.0.0:5041")

}
