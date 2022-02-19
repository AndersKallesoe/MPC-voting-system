use std::net::TcpStream;
use std::io::{self, Write};
use rand::prelude::*;

/*
    this method fullfills the client role in a secret sharing addition voting protocol:
    1. generate vote
    2. generate shares
    3. connect and communicate shares to server
*/
pub fn client(prime: u64, servers: Vec<&str>){
    let vote = random_vote();
    let share0 = random_share(prime);
    let share1 = (prime + vote - share0) % prime;
    connect_and_share(servers[0], share0);
    connect_and_share(servers[1], share1)
}

pub fn connect_to_server(addr: &str) -> TcpStream {
    TcpStream::connect(addr).expect("Error!")
}

pub fn connect_and_share(addr: &str, share: u64){
    let mut stream = connect_to_server(addr);
    stream.write(&share.to_be_bytes()).expect("Error");
}

/* generate random vote 1(yes) or 0(no) */
pub fn random_vote() -> u64{
    if rand::random(){ 1 } else { 0 }
}

/* generate a random share between 0 and prime */
pub fn random_share(prime: u64) -> u64{
    rand::random::<u64>() % prime
}