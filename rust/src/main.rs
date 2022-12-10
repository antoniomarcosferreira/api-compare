
use std::thread;
use std::time::Duration; 
use std::sync::{Mutex, Arc};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome Rust")
}

#[get("/sleep100")]
async fn sleep100() -> impl Responder {
    thread::sleep(Duration::from_millis(100));  
    HttpResponse::Ok().body(format!( "Rust 5*5 =  {}", 5*5))
}


#[get("/inc")]
async fn inc() -> impl Responder {
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
    HttpResponse::Ok().body(format!("Rust balance = {} ",  *safe.lock().unwrap()) )
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(inc)
            .service(sleep100)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 2000))?
    .run()
    .await
}
