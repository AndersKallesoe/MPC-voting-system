
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
mod test;
mod terminal_input;
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

/**
 * pretty print fault detection.
 */

fn main() {
    // let y_vec = vec![28, 18, 8];
    // let x_vec = vec![1, 2, 3];
    // println!("{:?}", lagrange::lagrange_coefficients(&x_vec, &y_vec, 29));
    // let shamir_protocol = Protocol{prime: 113, servers: 24, voters: 20, protocol: ProtocolType::Shamir};
    // run_protocol(shamir_protocol, vec![]);
    // let shamir = Protocol{prime: 29, servers: 4, voters: 10, protocol: ProtocolType::ShamirErrorCorrection};
    // test::run_and_report(shamir, vec![], true)
    terminal_input::run_system()
}

fn run_protocol(protocol: Protocol, corrupt: Vec<u8>, print: bool)-> (i64, Vec<i64>){
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
    let arc_server_list = server_list.clone();
    thread::spawn(
        move||{create_servers(main_server_address, protocol, corrupt, print)});
    let server_streams = listen_for_servers(server_listener, arc_server_list, protocol);
    let arc_server_list = server_list.clone(); 
    
    broadcast_server_list(&server_streams, arc_server_list);
    let arc_server_list = server_list.clone();

    thread::spawn(
            move ||{create_clients(arc_server_list,protocol, print)});
    let result = listen_for_clients(client_listener, protocol.voters);
    let results = get_results_from_servers(server_streams,protocol);
    (result, results)
}



fn broadcast_server_list(server_streams: &Vec<TcpStream>, arc_server_list: Arc<Mutex<Vec<(SocketAddrV4,SocketAddrV4)>>>){
    
    for mut stream in server_streams{
        let clone =  arc_server_list.clone();
        let guard = clone.lock().unwrap();
        let msg = serde_json::to_string(&guard[..]).unwrap();
        stream.write(&msg.as_bytes()).expect("failed to send serverlist");
    }
}

fn get_results_from_servers(server_streams: Vec<TcpStream>, protocol: Protocol)->Vec<i64>{
    let mut results = vec![0 as i64; protocol.servers as usize + 1];
    let mut responds = 0;
    for mut stream in server_streams{
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
        
        
    };
    vec![]
}

fn add_address(address: (SocketAddrV4, SocketAddrV4), arc_server_list: Arc<Mutex<Vec<(SocketAddrV4, SocketAddrV4)>>>){
    arc_server_list.lock().unwrap().push(address);
}
fn listen_for_servers(listener: TcpListener, arc_server_list: Arc<Mutex<Vec<(SocketAddrV4, SocketAddrV4)>>>, protocol: Protocol)-> Vec<TcpStream>{
    let mut servers = protocol.servers;
    let mut streams = vec![];
    for stream in listener.incoming(){
        match stream{
            Ok(mut stream) =>{
                let mut data = [0 as u8; 1024];
                match stream.read(&mut data){
                    Ok(size)=>{
                        let sent_str = from_utf8(&data[0..size]).unwrap();
                        let addr: (SocketAddrV4, SocketAddrV4) = serde_json::from_str(&sent_str).expect("Error serializing from json");
                        let clone = arc_server_list.clone();
                        add_address(addr, clone);
                        servers -=1;
                        streams.push(stream);
                        if servers == 0 {
                            return streams
                        }
                    }
                    Err(_)=>{}
                }
            }
            Err(err) => {println!("Error: {}", err); panic!();}
        }
    };
    vec![]
}

fn create_servers(main_addr: SocketAddrV4, protocol: Protocol, mut corrupt: Vec<u8>, print: bool){
    corrupt.reverse();
    let mut next_corrupt = get_next_corrupt(&mut corrupt);
    for i in 0..protocol.servers {
        let mut honest = true;
        if i==next_corrupt{ next_corrupt = get_next_corrupt(&mut corrupt);honest = false}
        thread::spawn(
            move ||{server::protocol_server(protocol.clone(), main_addr.clone(), honest, print)});
    }

fn get_next_corrupt(corrupt: &mut Vec<u8>) -> u8{
    match corrupt.pop(){
        None => {u8::MAX}
        Some(i) => {i}
    }
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
fn create_clients(server_list: Arc<Mutex<Vec<(SocketAddrV4, SocketAddrV4)>>>, protocol: Protocol, print: bool){
    let mut server_list_copy = vec![];
    for addr in server_list.lock().unwrap()[..].iter(){
        server_list_copy.push((*SocketAddrV4::ip(&addr.1),addr.1.port()));
    }
    for _ in 0..protocol.voters{
        let protocol_clone = protocol.clone();
        let clone = server_list_copy.clone();
        thread::spawn(
            move||{client::client(clone, protocol_clone, print)}
        );
    }
}