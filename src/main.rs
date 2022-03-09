use serde::{Serialize, Deserialize};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::{thread, time};
use std::sync::{Arc, Mutex};
use num::rational::Ratio;

mod server;
mod server_;
mod client;
mod lagrange;

#[derive(Serialize, Deserialize, Debug)]
struct Test1{
    test2: Vec<String>
}

fn main() {
    let strat = server_::Additive;
    let s = server_::SendServer{send: Arc::new(Mutex::new(strat))};
    s.start_server(vec!["1", "2", "3"], 2);
}

fn vote(prime: u64, voters: u64){
    //thread::spawn(move || server::start_server(["127.0.0.1:3333", "127.0.0.1:3335"],0,prime));
    //thread::spawn(move || server::start_server(["127.0.0.1:3333", "127.0.0.1:3335"],1,prime));
    for _ in 0..voters{  
        client::client(prime, ["127.0.0.1:3333", "127.0.0.1:3335"]);
    }
    let five_secs = time::Duration::from_secs(5);
    thread::sleep(five_secs);
}

