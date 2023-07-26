package main

import (
	"fmt"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"wagewhiz.nurliman.dev/action-catcher/env"
	"wagewhiz.nurliman.dev/action-catcher/handlers"
)

func main() {
	envVar := env.GetEnv()
	e := echo.New()

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	e.GET("/v0/job-titles/search", handlers.SeachJobTitles)
	e.POST("/v0/job-titles/:id/pick", handlers.UserPickJobTitle)

	addr := fmt.Sprintf("%s:%d", envVar.Host, envVar.Port)
	e.Logger.Fatal(e.Start(addr))
}
