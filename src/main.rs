extern crate hyper;

use std::env;

mod request;
use request::*;

fn main() {
    let url = env::args().nth(1).unwrap();
    let mut request = Request::new();

    println!("Request: {:?}", url);
    println!("Response: {:?}", request.get(&url));
}
