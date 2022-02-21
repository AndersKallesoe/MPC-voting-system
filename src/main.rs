
use std::{thread, time};
mod server;
mod client;

fn main() {
    vote(23,20);
}

fn vote(prime: u64, voters: u64){
    thread::spawn(move || server::start_server(["127.0.0.1:3333", "127.0.0.1:3335"],0,prime));
    thread::spawn(move || server::start_server(["127.0.0.1:3333", "127.0.0.1:3335"],1,prime));
    for _ in 0..voters{  
        client::client(prime, ["127.0.0.1:3333", "127.0.0.1:3335"]);
    }
    let five_secs = time::Duration::from_secs(5);
    thread::sleep(five_secs);
}