extern crate hyper;

mod request;
use request::*;

fn main() {
    // @TODO get url params from CLI
    let test_uri = "http://www.google.com";
    let mut request = Request::new();

    println!("Request: {:?}", test_uri);
    println!("Response: {:?}", request.get(test_uri));
}
