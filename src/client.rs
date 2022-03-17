
// use std::net::TcpStream;
// use std::io::{Write};

// struct ShamirClient;
// struct AdditiveClient{
//     servers: Vec<&'static str>
// }

// impl AdditiveClient{
//     pub fn test1(){}
//     fn test2(){}
// }

// trait Client{
//     fn broadcast(&self, vote: i64, shares: Vec<i64>);
//     fn create_votes_and_shares(&self, prime: i64) -> (i64, Vec<i64>);
// }

// impl Client for ShamirClient {
//     fn broadcast(&self, vote: i64, shares: Vec<i64>){}
//     fn create_votes_and_shares(&self, prime: i64) -> (i64, Vec<i64>){(0, vec![])}
// }

// impl Client for AdditiveClient {
//     fn broadcast(&self, vote: i64, shares: Vec<i64>){
//         connect_and_share(self.servers[0],vote);
//         let addrs = &self.servers[1..self.servers.len()-1];
//         for (addr, share) in addrs.iter().zip(shares.iter()){
//             connect_and_share(addr, *share);
//         }
//     }
    
//     fn create_votes_and_shares(&self, prime: i64) -> (i64, Vec<i64>){
//         let sum = 0;
//         let n = self.servers.len()-1;
//         let vote = random_vote();
//         let shares: Vec<i64> = vec![];
//         for _ in 0..n-1{
//             let share = random_share(prime);
//             shares.push(share);
//             sum += share;
//         }
//         let last_share = (prime + vote - (sum % prime)) % prime;
//         shares.push(last_share);
//         (vote, shares)
//         // let vote = random_vote();
//         // let share0 = random_share(prime);
//         // let share1 = (prime + vote - share0) % prime;
//         // println!("Vote: {}, share0: {}, share1: {}", vote, share0, share1);
//         // connect_and_share(servers[0], share0);
//         // connect_and_share(servers[1], share1);
//     }
// }

// impl dyn Client{
//     pub fn connect_to_server(addr: &str) -> TcpStream {
//         TcpStream::connect(addr).expect("Error connecting to server")
//     }

// }
// /*
//     this method fullfills the client role in a secret sharing addition voting protocol:
//     1. generate vote
//     2. generate shares
//     3. connect and communicate shares to server
// */
// pub fn client(prime: i64, servers: [&str; 2]){
//     let vote = random_vote();
//     let share0 = random_share(prime);
//     let share1 = (prime + vote - share0) % prime;
//     println!("Vote: {}, share0: {}, share1: {}", vote, share0, share1);
//     connect_and_share(servers[0], share0);
//     connect_and_share(servers[1], share1);

// }

// pub fn connect_to_server(addr: &str) -> TcpStream {
//     TcpStream::connect(addr).expect("Error connecting to server")
// }

// pub fn connect_and_share(addr: &str, share: i64){
//     let mut stream = connect_to_server(addr);
//     stream.write(&share.to_le_bytes()).expect("Error");
// }

// /* generate random vote 1(yes) or 0(no) */
// pub fn random_vote() -> i64{
//     if rand::random(){ 1 } else { 0 }
// }

// /* generate a random share between 0 and prime */
// pub fn random_share(prime: i64) -> i64{
//     rand::random::<i64>() % prime
// }