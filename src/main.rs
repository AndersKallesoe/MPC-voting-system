
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

/**
 * pretty print fault detection.
 */

fn main() {
    // let additive_2_protocol = Protocol{prime: 29, servers: 2, voters: 20, protocol: ProtocolType::Additive};
    // run_protocol(additive_2_protocol, vec![]);
    // let additive_3_protocol = Protocol{prime: 113, servers: 10, voters: 100, protocol: ProtocolType::Additive};
    // run_protocol(additive_3_protocol, vec![]);
    // let shamir_protocol = Protocol{prime: 29, servers: 5, voters: 20, protocol: ProtocolType::Shamir};
    // run_protocol(shamir_protocol, vec![])
    let corrupt = vec![2];
    let shamir_protocol = Protocol{prime: 17, servers: 3, voters: 5, protocol: ProtocolType::ShamirFaultDetection};
    run_protocol(shamir_protocol, corrupt);
}

fn run_protocol(protocol: Protocol, corrupt: Vec<u8>){
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
    println!("creating servers...");
    thread::spawn(
        move||{create_servers(main_server_address, protocol, corrupt)});
    let server_streams = listen_for_servers(server_listener, arc_server_list, protocol);
    let arc_server_list = server_list.clone(); 
    
    broadcast_server_list(&server_streams, arc_server_list);
    let arc_server_list = server_list.clone();
    println!("creating clients...");

    thread::spawn(
            move ||{create_clients(arc_server_list,protocol)});
    println!("collecting votes...");
    let result = listen_for_clients(client_listener, protocol.voters);
    println!("collecting results...");
    let results = get_results_from_servers(server_streams,protocol);
    report_results(protocol, result, results);
}

fn report_results(protocol: Protocol, result: i64, results: Vec<i64>){
    line();
    println!("Results:");
    line();
    println!("Protocol: {:?}",protocol.protocol);
    println!("Servers: {}", protocol.servers);
    println!("Voters: {}", protocol.voters);
    println!("Prime: {}", protocol.prime);
    line();
    println!("Actual Result: {}",result);
    println!("Server Results: {:?}", &results[1..]);
    line();
    let mut agree = true;
    let mut lastresult = results[1];
    for r in &results[1..] {
        if *r != lastresult{
            agree = false;
            break;
        }
        lastresult = *r;
    }
    if agree{
        println!("all servers agree");
         match results[1] {
            -1 => {println!("a fault was detected in the protocol")}
            -2 =>{println!("could not find polynomial consisting of integers!")}
            _=>{let success = check_results(result,results);
                println!("Protocol succes: {}",success);}
        };
        
    }else{
        println!("server disagree(there is a bug!)");
        println!("{:?}", results);
    }
    
    line();
}
fn line(){
    println!("________________________________________________________________________")
}
fn check_results(result: i64, results: Vec<i64>)->bool{
    for r in results[1..].iter(){
        if *r != result{
            return false
        }
    };
    return true
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

fn create_servers(main_addr: SocketAddrV4, protocol: Protocol, mut corrupt: Vec<u8> ){
    let mut next_corrupt = get_next_corrupt(&mut corrupt);
    for i in 0..protocol.servers {
        let mut honest = true;
        if i==next_corrupt{ next_corrupt = get_next_corrupt(&mut corrupt);honest = false}
        thread::spawn(
            move ||{server::protocol_server(protocol.clone(), main_addr.clone(), honest)});
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


