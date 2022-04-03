use std::net::{TcpListener, TcpStream,SocketAddrV4, SocketAddr};
use std::io::{Write, Read};
use std::{thread};
use std::sync::{Arc, Mutex};
use std::str::*;
use crate::*;

pub fn protocol_server(protocol: Protocol, mainaddr: SocketAddrV4, honest: bool){
    let server_listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let server_listener_addr = 
        match server_listener.local_addr() {
            Ok(SocketAddr::V4(ip4)) => ip4,
            _ => panic!()
        };
    let client_listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let client_listener_addr = 
        match client_listener.local_addr() {
            Ok(SocketAddr::V4(ip4)) => ip4,
            _ => panic!()
        };
    
    let shares = Arc::new(Mutex::new(vec![]));
    let arc_shares = shares.clone(); 
    let (main_stream,addr_list) = connect_to_main(mainaddr, (server_listener_addr, client_listener_addr));
    listen_for_clients(protocol.voters, client_listener,arc_shares);
    let sum = match honest{
        true => {{let shares_sum = sum(shares); println!("Honest {}", shares_sum); shares_sum}}
        false => {{let shares_sum = sum(shares) + 1; println!("Dishonest {}", shares_sum); shares_sum}}
    };
    let mut vec = vec![0 as i64; protocol.servers as usize + 1];
    vec[get_index_from_addr(&addr_list, server_listener_addr)] = sum;

    let sums = Arc::new(Mutex::new(vec));
    let arc_sums = sums.clone();
    let addr_list_clone = addr_list.clone();
    let listen_for_servers_thread = thread::spawn(
        move
        ||{listen_for_servers(server_listener,addr_list_clone, arc_sums, server_listener_addr)});
    
    let arc_sums = sums.clone();
    connect_to_servers(addr_list.clone(), arc_sums, server_listener_addr);
    let arc_sums = sums.clone();
    listen_for_servers_thread.join().expect("Couldn't join threads!");
    
    let secret = match protocol.protocol {
        ProtocolType::Additive => {
            additive::recover_secret(&arc_sums.lock().unwrap()[1..], protocol.prime)
        }
        ProtocolType::Shamir => {
            shamir::recover_secret(&arc_sums.lock().unwrap()[1..])
        }
        ProtocolType::ShamirFaultDetection=>{
            shamir::fault_detection(&arc_sums.lock().unwrap()[1..])
        }
        _ => {
            println!("pattern match failed");
            0
        }
    };
    let own_index = get_index_from_addr(&addr_list, server_listener_addr);
    send_result(main_stream, secret,own_index);
}

fn send_result(mut main_stream: TcpStream, secret: i64, own_index: usize){
    let msg = (own_index, secret); 
    let result_json = serde_json::to_string(&msg).unwrap();
    main_stream.write(result_json.as_bytes()).expect("Error writing to main!");
}

fn get_index_from_addr(addr_list: &Vec<(SocketAddrV4, SocketAddrV4)>, addr: SocketAddrV4) -> usize{
    for (i, (server_addr, _)) in addr_list.iter().enumerate(){
        if *server_addr == addr{return i}
    }
    println!("{} Address not found", addr);
    0
}

fn connect_to_servers(addr_list: Vec<(SocketAddrV4,SocketAddrV4)>, arc_sums: Arc<Mutex<Vec<i64>>>, own_addr: SocketAddrV4){
    
    let own_index = get_index_from_addr(&addr_list, own_addr);
    let clone = arc_sums.clone();
    let mut guard = clone.lock().unwrap();
    let own_sum = guard[own_index];
    let msg = (own_index, own_sum); 
    let sum_json = serde_json::to_string(&msg).unwrap();
    for i in 1..own_index {
        
        let mut stream = TcpStream::connect(addr_list[i].0).expect("could not connect to server");
        stream.write(sum_json.as_bytes()).expect("Error writing to main!");
        let mut data = [0 as u8; 1024];
        match stream.read(&mut data){
            Ok(size)=>{
                let sent_str = std::str::from_utf8(&data[0..size]).unwrap();
                let received_message: i64 = serde_json::from_str(&sent_str).expect("Error serializing from json");
                guard[i] = received_message;
            }
            Err(_)=>{ println!("")}
        }
    } 
}

fn listen_for_servers(listener: TcpListener, addr_list: Vec<(SocketAddrV4,SocketAddrV4)>, arc_sums: Arc<Mutex<Vec<i64>>>, own_addr: SocketAddrV4){
    let own_index = get_index_from_addr(&addr_list, own_addr);
    let mut remaining = addr_list.len() - 1 - own_index;
    if remaining == 0{return }
    let mut received_from = vec![false; addr_list.len()];
    for stream in listener.incoming(){
        match stream{
            Ok(mut stream) => {
                let mut sum_guard = arc_sums.lock().unwrap();
                let own_sum = sum_guard[own_index];
                let mut buff = [0 as u8; 1024];
                match stream.read(&mut buff){
                    Ok(size) => {
                        let received_str = from_utf8(&buff[0..size]).unwrap();
                        let received_message: (usize, i64) = serde_json::from_str(&received_str).unwrap();
                        if received_from[received_message.0] {continue}
                        received_from[received_message.0] = true;
                        sum_guard[received_message.0] = received_message.1;
                        let serialized_own_sum = serde_json::to_string(&own_sum).unwrap();
                        stream.write(&serialized_own_sum.as_bytes()).expect("could not write to stream");
                        remaining -= 1;
                        if remaining == 0{return }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
}



fn listen_for_clients(voters: u16, client_listener: TcpListener, arc_shares: Arc<Mutex<Vec<i64>>>){
    for stream in client_listener.incoming(){
        match stream{
            Ok(mut stream) =>{
                let mut data = [0 as u8; 1024];
                match stream.read(&mut data){
                    Ok(size)=>{
                        let sent_str = from_utf8(&data[0..size]).unwrap();
                        let share: i64 = serde_json::from_str(&sent_str).expect("Error serializing from json");
                        let clone = arc_shares.clone();
                        let mut guard = clone.lock().unwrap();
                        guard.push(share);
                        if guard.len() as u16 == voters{return}
                    }
                    Err(err)=>{println!("Server Error 1, {}", err);}
                }
            }
            Err(msg) => {println!("{}: Server Error 2",msg);}
        }
    }
}

fn connect_to_main(mainaddr: SocketAddrV4, server_listener_addr: (SocketAddrV4,SocketAddrV4))->(TcpStream,Vec<(SocketAddrV4, SocketAddrV4)>){
    let mut main_stream = TcpStream::connect(mainaddr).expect("could not connect to main server");
    let addr_json = serde_json::to_string(&server_listener_addr).unwrap();
    main_stream.write(addr_json.as_bytes()).expect("Error writing to main!");
    let mut data = [0 as u8; 1024];
    match main_stream.read(&mut data){
        Ok(size)=>{
            let sent_str = std::str::from_utf8(&data[0..size]).unwrap();
            let mut error_str = String::from("Error serializing string: ");
            error_str.push_str(sent_str);
            error_str.push_str("\n");
            //println!("Server: {}", sent_str);
            let addr_list: Vec<(SocketAddrV4, SocketAddrV4)> = serde_json::from_str(&sent_str).expect(&error_str);
            (main_stream, addr_list)
        }
        Err(_)=>{ println!("did not receive server list"); panic!()}
    }
}
fn sum (shares: Arc<Mutex<Vec<i64>>>)-> i64 {
    let mut sum = 0;
    for share in shares.lock().unwrap().iter() {
        sum += share;
    }
    sum
}