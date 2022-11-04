package main

import (
	"container/list"
	"log"
	"github.com/gofiber/fiber/v2"
)

type WeatherForecast struct {

}

func main() {
	app := fiber.New()

	app.Get("/weatherforecast", )
}