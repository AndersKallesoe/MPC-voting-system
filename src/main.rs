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
    /*
    let test1 = Test1{test2: vec![String::from("bruh1"), String::from("bruh2"), String::from("bruh3")]};
    let serialized = serde_json::to_string(&test1).unwrap();
    println!("Serialized: {}", serialized);
    let deserialized: Test1 = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized)
    */

    let X = &vec![Ratio::new(1, 1), Ratio::new(2, 1), Ratio::new(3, 1)];
    let C = &vec![Ratio::new(10, 1),Ratio::new(8, 1), Ratio::new(4, 1)];
    //let alpha = Ratio::new(zero, one);
    println!("{:?}",lagrange::lagrange_coefficients(X, C)); 
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

