
use serde::{Serialize, Deserialize};
use std::net::*;
use std::{thread};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use std::str::from_utf8;

mod server;
mod client;
mod lagrange;
mod shamir;
mod additive;
#[derive(Serialize, Deserialize, Debug)]

#[derive(Clone, Copy)]
pub enum ProtocolType{
    Additive,
    Shamir,
    ShamirFaultDetection,
    ShamirErrorCorrection
}


#[derive(Clone, Copy)]
pub struct Protocol{
    prime: i64,
    servers: u8,
    voters: u16,
    protocol: ProtocolType
}

fn main() {
    // let bytes = (256 as i64).to_le_bytes();
    // let vote: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 255];
    // let u = u64::from_le_bytes(vote);
    //println!("{:?}", bytes);
    // println!("{}", u);
    let protocol = Protocol{prime: 29, servers: 2, voters: 20, protocol: ProtocolType::Additive};
    run_protocol(protocol)
}

fn run_protocol(protocol: Protocol){
    let server_list = Arc::new(Mutex::new(vec![]));
    let server_listener = TcpListener::bind("127.0.0.1:0").expect("Error creating server listener!");
    let client_listener = TcpListener::bind("127.0.0.1:0").expect("Error creating client listener!");
    let main_server_address = match server_listener.local_addr() {
        Ok(SocketAddr::V4(ip4)) => ip4,
        _ => panic!()
    };
    let main_client_address = match client_listener.local_addr() {
        Ok(SocketAddr::V4(ip4)) => ip4,
        _ => panic!()
    };

    let arc_server_list = server_list.clone();
    add_address((main_server_address, main_client_address), arc_server_list);
    println!("{:?}", server_list.lock().unwrap());
    let arc_server_list = server_list.clone();

    thread::spawn(
        move||{create_servers(main_server_address, protocol)});
    thread::spawn(
        ||{listen_for_servers(server_listener, arc_server_list)});// thread

    loop{
        let arcclone = server_list.clone();
        let guard = arcclone.lock().unwrap();
        if guard.len() == 3{
            break;
        }
        std::mem::drop(guard);
    }
    thread::spawn(
            move ||{create_clients(server_list.clone(),protocol)});
    let result = listen_for_clients(client_listener, protocol.voters);
    println!("Result: {}", result);
    //get_results_from_servers();
    //report_results();
    loop{}
}

fn get_results_from_servers(listener: TcpListener, protocol: Protocol)->Vec<i64>{
    let mut results = vec![0 as i64; protocol.servers as usize + 1];
    let mut responds = 0;
    for stream in listener.incoming(){
        match stream{
            Ok(mut stream) =>{
                let mut data = [0 as u8; 1024];
                match stream.read(&mut data){
                    Ok(size)=>{
                        let sent_str = from_utf8(&data[0..size]).unwrap();
                        let (index,result): (usize, i64) = serde_json::from_str(&sent_str).expect("Error serializing from json");
                        results[index]=result;
                        responds += 1;
                        if responds == protocol.servers {
                            return results
                        }
                    }
                    Err(_)=>{println!("Main error 1");  return vec![]}
                }
            }
            Err(_) => {println!("Main error 2"); return vec![]}
        }
    };
    vec![]
}

fn add_address(address: (SocketAddrV4, SocketAddrV4), arc_server_list: Arc<Mutex<Vec<(SocketAddrV4, SocketAddrV4)>>>){
    arc_server_list.lock().unwrap().push(address);
}
fn listen_for_servers(listener: TcpListener, arc_server_list: Arc<Mutex<Vec<(SocketAddrV4, SocketAddrV4)>>>){
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
            Err(err) => {println!("Error: {}", err); panic!();}
        }
    }
    loop{}
}

fn handle_server(mut conn: TcpStream, arc_server_list: Arc<Mutex<Vec<(SocketAddrV4, SocketAddrV4)>>>){
    let mut data = [0 as u8; 1024];
    match conn.read(&mut data){
        Ok(size)=>{
            let sent_str = from_utf8(&data[0..size]).unwrap();
            let addr: (SocketAddrV4, SocketAddrV4) = serde_json::from_str(&sent_str).expect("Error serializing from json");
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

fn create_servers(main_addr: SocketAddrV4, protocol: Protocol){
    for _ in 0..protocol.servers {
        thread::spawn(
            move ||{server::protocol_server(protocol.clone(), main_addr.clone())});
    }
    
}
fn listen_for_clients(listener: TcpListener, voters: u16) -> i64{
    let mut votes:i64 = 0;
    let mut result:i64 = 0;
    for stream in listener.incoming(){
        match stream{
            Ok(mut stream) =>{
                let mut data = [0 as u8; 1024];
                match stream.read(&mut data){
                    Ok(size)=>{
                        let sent_str = from_utf8(&data[0..size]).unwrap();
                        let vote: i64 = serde_json::from_str(&sent_str).expect("Error serializing from json");
                        votes += 1;
                        result += vote;
                        if votes == (voters as i64) {
                            return result
                        }
                    }
                    Err(_)=>{}
                }
            }
            Err(_) => {println!("Main error 3"); panic!();}
        }
    }
    loop{}
}
fn create_clients(server_list: Arc<Mutex<Vec<(SocketAddrV4, SocketAddrV4)>>>, protocol: Protocol){
    let mut server_list_copy = vec![];
    for addr in server_list.lock().unwrap()[..].iter(){
        server_list_copy.push((*SocketAddrV4::ip(&addr.1),addr.1.port()));
    }
    for _ in 0..protocol.voters{
        let protocol_clone = protocol.clone();
        let clone = server_list_copy.clone();
        thread::spawn(
            move||{client::client(clone, protocol_clone)}
        );
    }
}


