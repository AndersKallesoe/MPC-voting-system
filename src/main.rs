
use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::net::*;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use num::rational::Ratio;
use std::io::{Read, Write};
use std::str;

mod server;
mod server_;
mod client;
mod lagrange;
mod server2;
#[derive(Serialize, Deserialize, Debug)]
struct Test1{
    test2: Vec<String>
}

fn main() {
    run_protocol()
}



fn run_protocol(){
    let server_list = Arc::new(Mutex::new(vec![]));
    let server_listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    let main_address = match server_listener.local_addr() {
        Ok(SocketAddr::V4(ip4)) => ip4,
        _ => panic!()
    };
    let arc_1 = server_list.clone();
    add_address(main_address, arc_1);
    println!("{:?}", server_list.lock().unwrap());
    let arc_2 = server_list.clone();
    thread::spawn(
        ||{listen_for_servers(server_listener, arc_2)});// thread
    create_servers();
    thread::spawn(
        ||{listen_for_clients()});
    create_clients();
    loop{
        //if results.lock().unwrap().len() >= server_list.lock().unwrap().len()-1{
        //   check_results()
        //}
    }
}
fn add_address(address: SocketAddrV4,arc_server_list: Arc<Mutex<Vec<SocketAddrV4>>>){
    arc_server_list.lock().unwrap().push(address);
}
fn listen_for_servers(listener: TcpListener, arc_server_list: Arc<Mutex<Vec<SocketAddrV4>>>){
    
    for stream in listener.incoming(){
        match stream{
            Ok(mut stream) =>{
                let arc = arc_server_list.clone();
                thread::spawn(
                    move || {
                       handle_server(
                            &mut stream,arc
                        )
                    }
                );
            }
            Err(_) => {println!("Error!"); panic!();}
        }
    }
    loop{}
}

fn handle_server(conn: &mut TcpStream, arc_server_list: Arc<Mutex<Vec<SocketAddrV4>>>){
    let mut data = [0 as u8; 1024];
    match conn.read(&mut data){
        Ok(size)=>{
            //let share = Ratio::new(i64::from_le_bytes(data), 1);
            let sent_str = str::from_utf8(&data[0..size]).unwrap();
            let addr: SocketAddrV4 = serde_json::from_str(&sent_str).expect("Error serializing from json");
            println!("main: HERE");
            let clone = arc_server_list.clone();
            let mut guard = clone.lock().unwrap();
            guard.push(addr);
            println!("main: [{},{}]",guard[0],guard[1])
        }
        Err(_)=>{}
    }
    // vent paa den faar serveres resultat
    // naar resultat er modtaget check om alle resultater er modtaget: hvis de er afslut
    
} 
fn create_servers(){
    server2::protocol_server(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3333))
}
fn listen_for_clients(){

}
fn create_clients(){

}
fn check_results(){
    //check at results og votes stemmer overens
}


