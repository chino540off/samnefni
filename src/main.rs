extern crate serde;

mod config;
mod model;

fn main() {
    let conf = config::Config::new();
    println!("{:?}", conf);
}
