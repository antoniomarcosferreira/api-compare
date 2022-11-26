import time
from flask import Flask
from threading import Thread

app = Flask(__name__)
@app.route('/')
def hello_world():  # put application's code here
    return 'Welcome flask'

@app.route('/sleep100')
def process1():  # put application's code here
    time.sleep(100/1000)
    m = 5 * 5
    return "Flesk 5*5 = {} ".format(m)

# thread unsafe counter class
class ThreadUnsafeCounter():
    # constructor
    def __init__(self):
        # initialize counter
        self._counter = 0
 
    # increment the counter
    def increment(self):
        time.sleep(100 / 1000)
        self._counter += 1
 
    # get the counter value
    def value(self):
        return self._counter


# task executed by threads
def task(counter):
    counter.increment()


@app.route('/inc')
def inc():
    # create the counter
    counter = ThreadUnsafeCounter()
    # create 10 threads to increment the counter
    threads = [Thread(target=task, args=(counter,)) for _ in range(50)]
    # start all threads
    for thread in threads:
        thread.start()
    # wait for all threads to finish
    for thread in threads:
        thread.join()
    # report the value of the counter
    print(counter.value())
        
    return f'Flask balance {counter.value()}'


if __name__ == '__main__':
	app.run(host='0.0.0.0', port=5000)
