use hyper::Client;
use hyper::header::Connection;
use std::io::Read;

pub struct Request {
    response: String
}

impl Request {
    pub fn new() -> Request {
        Request {
            response: "".to_owned(),
        }
    }

    pub fn get(&mut self, uri: &str) -> String {
        let client = Client::new();

        let mut res = client.get(uri)
            .header(Connection::close())
            .send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        self.response = body.clone();
        body
    }
}
