#[macro_use]
extern crate serde_derive;

mod api;
mod models;
mod utils;

use dotenv::dotenv;
use std::net::{Shutdown, TcpListener};
use std::thread;

fn main() {
  dotenv().ok();

  let listener = TcpListener::bind("0.0.0.0:8888").unwrap();
  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        thread::spawn(move || {
          api::router(&stream);
          stream.shutdown(Shutdown::Both).unwrap();
        });
      }
      Err(e) => {
        panic!("{}", e);
      }
    }
  }
  drop(listener);
}
