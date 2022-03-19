
use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::net::*;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use num::rational::Ratio;
use std::io::{Read, Write};
use std::str::from_utf8;

mod server;
mod client;
mod lagrange;
mod shamir;
mod additive;
#[derive(Serialize, Deserialize, Debug)]

pub enum Protocol{
    Additive{prime: i64},
    Shamir{prime: i64},
    ShamirFaultDetection{prime: i64},
    ShamirErrorCorrection{prime: i64}
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

    let arc_server_list = server_list.clone();
    add_address(main_address, arc_server_list);
    println!("{:?}", server_list.lock().unwrap());
    let arc_server_list = server_list.clone();


    thread::spawn(
        ||{listen_for_servers(server_listener, arc_server_list)});// thread
    thread::spawn(
        ||{create_servers()});

    loop{
        let arcclone = server_list.clone();
        let guard = arcclone.lock().unwrap();
        if guard.len() == 3{
            break;
        }
        std::mem::drop(guard);
    }


    

    let votes = Arc::new(Mutex::new(vec![]));
    let arc_votes = votes.clone();
    thread::spawn(
        ||{listen_for_clients(arc_votes)});
    let protocol = Protocol::Additive{prime: 17};
    
    //let server_list_clone = serve
    thread::spawn(
        move ||{create_clients(server_list.clone(),protocol)});

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
            Ok(stream) =>{
                let arc = arc_server_list.clone();
                thread::spawn(
                    move || {
                       handle_server(
                            stream,arc
                        )
                    }
                );
            }
            Err(_) => {println!("Error!"); panic!();}
        }
    }
    loop{}
}

fn handle_server(mut conn: TcpStream, arc_server_list: Arc<Mutex<Vec<SocketAddrV4>>>){
    let mut data = [0 as u8; 1024];
    match conn.read(&mut data){
        Ok(size)=>{
            let sent_str = from_utf8(&data[0..size]).unwrap();
            let addr: SocketAddrV4 = serde_json::from_str(&sent_str).expect("Error serializing from json");
            let clone = arc_server_list.clone();
            println!("calling add_address");
            add_address(addr, clone);
            println!("main received adress from server");
            loop{
                let arcclone = arc_server_list.clone();
                let guard = arcclone.lock().unwrap();
                if guard.len() == 3{
                    let addr_json = serde_json::to_string(&guard[..]).unwrap();
                    //println!("{:?}",addr_json);
                    conn.write(addr_json.as_bytes()).expect("Error writing to server!");
                    //std::mem::drop(guard);
                    break;
                }
                std::mem::drop(guard);
            }


        }
        Err(_)=>{}
    }
    // vent paa den faar serveres resultat
    // naar resultat er modtaget check om alle resultater er modtaget: hvis de er afslut
    
} 
fn create_servers(){
    thread::spawn(
        ||{server::protocol_server(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3333))});
    thread::spawn(
        ||{server::protocol_server(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3333))});
}
fn listen_for_clients(arc_votes: Arc<Mutex<Vec<i64>>>){
    let listener = TcpListener::bind("127.0.0.1:3334").unwrap();
    for stream in listener.incoming(){
        match stream{
            Ok(mut stream) =>{
                let mut data = [0 as u8; 1024];
                match stream.read(&mut data){
                    Ok(size)=>{
                        let sent_str = from_utf8(&data[0..size]).unwrap();
                        println!("main: {}",sent_str);
                        let vote: i64 = serde_json::from_str(&sent_str).expect("Error serializing from json");
                        let clone = arc_votes.clone();
                        clone.lock().unwrap().push(vote);
                    }
                    Err(_)=>{}
                }
            }
            Err(_) => {println!("Error!"); panic!();}
        }
    }
    loop{}
}
fn create_clients(server_list: Arc<Mutex<Vec<SocketAddrV4>>>, protocol: Protocol){
    let mut server_list_copy = vec![];
    for addr in server_list.lock().unwrap()[..].iter(){
        server_list_copy.push((*SocketAddrV4::ip(&addr),addr.port()));
    }
    client::client(server_list_copy, protocol)
}

fn check_results(){
    //check at results og votes stemmer overens
}


