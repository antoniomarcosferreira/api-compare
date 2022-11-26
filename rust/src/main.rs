#[macro_use] extern crate nickel;
use near_sdk::{log, near_bindgen};
use std::{thread, time};  
use std::time::Duration; 
use std::sync::{Mutex, Arc};


use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new(); 

    server.get("/",  middleware! { |_req, _res| 
        "Welcome Rust"
    });

    server.get("sleep100",  middleware! { |_req, _res| 
        thread::sleep(Duration::from_millis(100));
        "Rust 5*5 = " + (5*5)
    }); 

    server.get("/inc", middleware! { |_request, response| 
        let safe = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _i in 0..50 {
            let safe = Arc::clone(&safe);
            let handle = thread::spawn(move|| { 
                thread::sleep(Duration::from_millis(100));
                let mut a = safe.lock().unwrap();
                *a += 1;
            });
            handles.push(handle);
        }

        // join the handles in the vector
        for i in handles {
            i.join().unwrap();
        }
    
        format!("Rust balance = {} ",  *safe.lock().unwrap()) 
    });

    server.listen("0.0.0.0:2000").unwrap();
}
 

#[near_bindgen]
pub struct Counter {
    val: i8,
}
 
 
#[near_bindgen]
impl Counter { 
  

    // Public read-only method: Returns the counter value.
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    // Public method: Increment the counter.
    pub fn increment(&mut self) {  
        let hundred_millis = time::Duration::from_millis(100);

        thread::sleep(hundred_millis);
        self.val += 1;
        log!("Increased number to {}", self.val);
    }

    // Public method: Decrement the counter.
    pub fn decrement(&mut self) {
        self.val -= 1;
        log!("Decreased number to {}", self.val);
    }

    // Public method - Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        log!("Reset counter to zero");
    }
}