extern crate iron;

use iron::prelude::*;
use iron::status::Status;

fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((Status::Ok, "Hello world!")))
    }).http("localhost:3500");
}