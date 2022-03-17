use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::{thread, time};
use std::sync::{Arc, Mutex};
use num::rational::Ratio;
use std::io::{Read, Write};

mod server;
mod server_;
mod client;
mod shamir;
mod lagrange;

#[derive(Serialize, Deserialize, Debug)]
struct Test1{
    test2: Vec<String>
}

fn main() {
    /*let strat = server_::Additive;
    let s = server_::SendServer{send: Arc::new(Mutex::new(strat))};
    s.start_server(vec!["1", "2", "3"], 2);*/
    let secret = 2;
    let t = 4;
    let prime = 17;
    let alphas = vec![1, 2, 3, 4, 5];
    let secret_func = shamir::Shamir::create_secret(secret, t, prime);
    let shares = shamir::Shamir::create_shares(&secret_func, &alphas);
    let recovered_secret = shamir::Shamir::recover_secret(&shares, &alphas);
    let recovered_coeffs = shamir::Shamir::recover_coefficients(&shares, &alphas);
    println!("Secret coefficients: {:?}", secret_func);
    println!("Shares: {:?}", shares);
    println!("Recovered secret: {:?}", recovered_secret);
    println!("Recovered coefficients: {:?}", recovered_coeffs);
}

/*fn vote(prime: u64, voters: u64){
    //thread::spawn(move || server::start_server(["127.0.0.1:3333", "127.0.0.1:3335"],0,prime));
    //thread::spawn(move || server::start_server(["127.0.0.1:3333", "127.0.0.1:3335"],1,prime));
    for _ in 0..voters{  
        client::client(prime, ["127.0.0.1:3333", "127.0.0.1:3335"]);
    }
    let five_secs = time::Duration::from_secs(5);
    thread::sleep(five_secs);
}*/
pub struct Main{
    votes: Arc<Mutex<Vec<i64>>>,
    results: Arc<Mutex<Vec<i64>>>,
    server_list: Arc<Mutex<Vec<String>>>
}

// impl Main{
//     fn run_protocol(&'static self){
        
//         thread::spawn(
//             ||{self.connect_servers()});// thread
//         self.create_servers();
//         thread::spawn(
//             ||{self.listen_for_clients()});
//         self.create_clients();
//         loop{
//             if self.results.lock().unwrap().len() >= self.server_list.lock().unwrap().len()-1{
//                 self.check_results()
//             }
//         }
//         // listener i thread: thread 
//         // skaber server objekter
//         // client listener i thread
//         // skaber client object
//         // når alle servere har givet resultat -> print rapport
//     }
//     fn connect_servers(&'static self){
//         let mut conns = vec![];
//         let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
//         for stream in listener.incoming(){
//             match stream{
//                 Ok(stream) =>{
//                     conns.push(stream);
//                     thread::spawn(
//                         || {
//                             Main::handle_server(
//                                 &stream
//                             )
//                         }
//                     );
//                 }
//                 Err(_) => {println!("Error!"); panic!();}
//             }
//         }
//         loop{}
//     }

//     fn handle_server(conn: &TcpStream){
//         let mut data = [0 as u8; 8];
//         match conn.read(&mut data){
//             Ok(size)=>{
//                 let share = Ratio::new(i64::from_le_bytes(data), 1);
//             }
//             Err(_)=>{}
//         }
//         // vent paa den faar serveres resultat
//         // naar resultat er modtaget check om alle resultater er modtaget: hvis de er afslut
        
//     } 
//     fn create_servers(&self){
//         //skab serverne, giv dem hardcoded ip og port
//     }
//     fn listen_for_clients(&self){

//     }
//     fn create_clients(&self){

//     }
//     fn check_results(&self){
//         //check at results og votes stemmer overens
//     }
    
// }

/*
    main server: hardcoded ip og port
    skaber servere, forskellige metoder til forskellige tests og scenarier.
    venter paa connection fra servere
    naar alle servere er connected: sender liste af alle servere til alle servere
    vent-- (eller klarmelding fra servere)
    skaber clienter
    venter paa connection fra clienter: indsamler alle votes uden de er secret
    venter paa connection fra servers: indsamler deres resultater
    afstemmer resultater
    printer resultat
*/

