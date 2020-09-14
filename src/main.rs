use actix_web::{App, http, server, HttpRequest};
use rand::prelude::*;


fn index(_req: HttpRequest) -> String {
    format!("Hi there lovely customer. We're still up!")
}

fn maybe_panic(_req: HttpRequest) -> String {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    if  y > 0.5 {
        let x: &i64 = unsafe { std::mem::transmute(0 as i64) };
        panic!("Sorry, not sorry. {}", x);
    }
    else {
        format!("Not yet panic-ing.")
    }
}

fn main() {
    server::new(|| App::new()
    .route("/", http::Method::GET, index)
    .route("/maybePanic", http::Method::GET, maybe_panic))
        .bind("0.0.0.0:8080").unwrap()
        .run();
    dbg!("Started k8s.");
}