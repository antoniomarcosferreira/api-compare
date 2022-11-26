package main

import (
	"time"
	"fmt" 
)

type Wallet struct {
	balance int
}

func NewWallet() *Wallet {
	return &Wallet{
		balance: 0,
	}
}
 
func (b *Wallet) Balance() int {
	fmt.Println("b.balance ", b.balance )
	return b.balance
}

func (b *Wallet) Deposit(amount int) {
	time.Sleep(time.Millisecond * 100)
	fmt.Println("Incrementou ", b.balance, amount )
	b.balance += amount
}
