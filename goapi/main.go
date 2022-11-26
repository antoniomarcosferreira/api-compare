package main

import (
	"fmt"
	"github.com/gofiber/fiber/v2"
     "sync"
	"time"
)

func main() {
	app := fiber.New()

	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("Welcome Go")
	})

	app.Get("/sleep100", func(c *fiber.Ctx) error {
		time.Sleep(time.Millisecond * 100)
		return c.SendString(fmt.Sprintf( "Go 5*5 = %d", 5*5))
	})

	app.Get("/inc", func(c *fiber.Ctx) error {
		bitcoin := NewWallet()
		wg := sync.WaitGroup{}
		for i := 0; i < 50; i++ {
			wg.Add(1)
			 go func() {
				bitcoin.Deposit(1)
				wg.Done()
			}() 
		}
		wg.Wait()
		return c.SendString(fmt.Sprintf("Go balance = %d",  bitcoin.Balance()))
	})

	app.Listen(":9000")
}
