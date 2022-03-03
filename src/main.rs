use serde::{Serialize, Deserialize};
use std::{thread, time};
use num::rational::Ratio;

mod server;
mod client;
mod lagrange;

#[derive(Serialize, Deserialize, Debug)]
struct Test1{
    test2: Vec<String>
}

fn main() {
    let test1 = Test1{test2: vec![String::from("bruh1"), String::from("bruh2"), String::from("bruh3")]};
    let serialized = serde_json::to_string(&test1).unwrap();
    println!("Serialized: {}", serialized);
    let deserialized: Test1 = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized)
    /* 
    let zero: i64 = 0;
    let one: i64 = 1;
    let two: i64 = 2;
    let three: i64 = 3;
    let X = &vec![Ratio::new(one, one), Ratio::new(two, one)];
    let C = &vec![Ratio::new(two,one),Ratio::new(three,one)];
    let alpha = Ratio::new(zero, one);
    println!("{}",lagrange::lagrange_interpolation(X, C, alpha)); */
    //vote(23,20);
}

//fn vote(prime: u64, voters: u64){
//    thread::spawn(move || server::start_server(["127.0.0.1:3333", "127.0.0.1:3335"],0,prime));
//    thread::spawn(move || server::start_server(["127.0.0.1:3333", "127.0.0.1:3335"],1,prime));
//    for _ in 0..voters{  
//        client::client(prime, ["127.0.0.1:3333", "127.0.0.1:3335"]);
//    }
//    let five_secs = time::Duration::from_secs(5);
//    thread::sleep(five_secs);
//}

