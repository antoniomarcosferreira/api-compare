package com.example.demo;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

@RestController
@SpringBootApplication
public class Demo3Application {

    @RequestMapping("/")
    String Hello() {
        return "Welcome Java";
    }

    @RequestMapping("/sleep100")
    String Process1() throws InterruptedException {
        TimeUnit.MILLISECONDS.sleep(100);
        return "Java 5*5 = " + (5 * 5);
    }

    @RequestMapping("/inc")
    String Inc() throws InterruptedException {
        Wallet bitcoin = new Wallet();
        int numberOfTasks = 50;

        CountDownLatch latch = new CountDownLatch(numberOfTasks);
        for (int i = 0; i < numberOfTasks; i++) {
            MyRunnable r = new MyRunnable(bitcoin, latch);
            r.start();
        }
        latch.await();
        latch.countDown();

        return "Java balance " + bitcoin.Balance();
    }

    public static void main(String[] args) {
        SpringApplication.run(Demo3Application.class, args);
    }

}

class MyRunnable extends Thread {
    Wallet bitcoin;
    private CountDownLatch latch;

    public MyRunnable(Wallet bitcoin, CountDownLatch latch) {
        this.bitcoin = bitcoin;
        this.latch = latch;
    }

    public void run() {
        try {
            bitcoin.Deposit(1);
            latch.countDown();
        } catch (Exception err) {
            err.printStackTrace();
        }
    }
}
