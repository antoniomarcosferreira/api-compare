# This is a simple test to consume APIs.

### The objective is to validate Golang in front of others languages focusing on performance.


___




## To run

### Start Docker processes (build and up)

    ./builder-all.sh
    ./start-all.sh

### Run the tests

    cd tests


test1 runs a single / endpoint to receive a Hello World across all APIs. There is no process on this endpoint. This test waits 3 seconds to time out of the request.

    ./test1.sh


The test2 runs a single endpoint /sleep100 to receive a 5*5 results in all APIs. No have process in this endpoint. This test waits 3 seconds to time out of the request.

    ./test2.sh

This endpoint consists of incrementing a variable called balance in a loop of iterations from 0 to 50.
However, to simulate a more real process with database, etc., at each iteration, the system makes a delay of 100 microseconds. Altogether there are 50 iterations. Endpoint response must be <Language name> balance = 50, example: "Go balance = 50".
This test waits 30 seconds to time out of the request.

    ./test3.sh

Same the test2 except the time out limit.
This test waits 3 seconds to time out of the request.

    ./test4.sh

### Finish all docker containers
    ./stop-all.sh





---

## Help me

Sorry if you implemented something wrong. Feel free to download the repository and improve anything in this test.