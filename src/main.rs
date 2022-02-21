use std::thread;

mod server;
mod client;

fn main() {
    //println!("Hello, world!");
    thread::spawn(move || server::start_server("127.0.0.1:3333", 5));
    thread::spawn(move || server::start_server("127.0.0.1:3334", 5));

    client::client(19, &vec!["127.0.0.1:3333", "127.0.0.1:3334"]);
    client::client(19, &vec!["127.0.0.1:3333", "127.0.0.1:3334"]);
    client::client(19, &vec!["127.0.0.1:3333", "127.0.0.1:3334"]);
    client::client(19, &vec!["127.0.0.1:3333", "127.0.0.1:3334"]);
    client::client(19, &vec!["127.0.0.1:3333", "127.0.0.1:3334"]);
    
}
