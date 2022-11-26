package com.example.demo;

import java.util.concurrent.TimeUnit;

public class Wallet {

    int balance;

    public Wallet() {
        balance = 0;
    }

    public int Balance() {
       return balance;
    }

    public void Deposit(int amount) throws InterruptedException {
        TimeUnit.MILLISECONDS.sleep(100);
        balance += amount;
        System.out.println("Incrementou ");
    }

}
