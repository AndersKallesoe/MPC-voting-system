use std::net::TcpStream;
use std::io::{Write};

struct ShamirClient;
struct AdditiveClient;

impl AdditiveClient{
    pub fn test1(){}
    fn test2(){}
}

trait Client{
    fn connect_and_share(&self, addrs: &Vec<&str>, shares: &Vec<f64>);
    fn create_votes_and_shares(&self) -> (f64, Vec<f64>);
}

impl Client for ShamirClient {
    fn connect_and_share(&self, addrs: &Vec<&str>, shares: &Vec<f64>){}
    fn create_votes_and_shares(&self) -> (f64, Vec<f64>){(0.0, vec![])}
}

impl Client for AdditiveClient {
    fn connect_and_share(&self, addrs: &Vec<&str>, shares: &Vec<f64>){
        for (addr, share) in addrs.iter().zip(shares.iter()){
            //connect_and_share(addr, share);
        }
    }
    fn create_votes_and_shares(&self) -> (f64, Vec<f64>){(0.0, vec![])}
}

impl dyn Client{
    pub fn connect_to_server(addr: &str) -> TcpStream {
        TcpStream::connect(addr).expect("Error connecting to server")
    }

}
/*
    this method fullfills the client role in a secret sharing addition voting protocol:
    1. generate vote
    2. generate shares
    3. connect and communicate shares to server
*/
pub fn client(prime: u64, servers: [&str; 2]){
    let vote = random_vote();
    let share0 = random_share(prime);
    let share1 = (prime + vote - share0) % prime;
    println!("Vote: {}, share0: {}, share1: {}", vote, share0, share1);
    connect_and_share(servers[0], share0);
    connect_and_share(servers[1], share1);

}

pub fn connect_to_server(addr: &str) -> TcpStream {
    TcpStream::connect(addr).expect("Error connecting to server")
}

pub fn connect_and_share(addr: &str, share: u64){
    let mut stream = connect_to_server(addr);
    stream.write(&share.to_le_bytes()).expect("Error");
}

/* generate random vote 1(yes) or 0(no) */
pub fn random_vote() -> u64{
    if rand::random(){ 1 } else { 0 }
}

/* generate a random share between 0 and prime */
pub fn random_share(prime: u64) -> u64{
    rand::random::<u64>() % prime
}