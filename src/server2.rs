use std::net::{TcpListener, TcpStream, Shutdown,SocketAddrV4,Ipv4Addr, SocketAddr};
use std::io::{Write, Read};
use std::{thread, time};

pub fn protocol_server(mainaddr: SocketAddrV4){
    //server lytter efter servere y
    //server lytter efter clienter y
    //server forbinder til main server; sender ip og port; venter paa liste af servere
    //server forbinder til relevante protocolservere
    //server modtager besked om afstemningen er slut fra main server
    //server adder shares og deler med protokolservere
    //naar shares == len (serverlist) koer protokol (parameter)
    //del resultat med main server og doe
    
    let server_listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let server_listener_addr = 
        match server_listener.local_addr() {
            Ok(SocketAddr::V4(ip4)) => ip4,
            _ => panic!()
        };
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let client_listener_addr =  SocketAddrV4::new(ip,server_listener_addr.port()+1);
    let client_listener = TcpListener::bind(client_listener_addr).unwrap();
    thread::spawn(
        move
        ||{listen_for_servers(server_listener)});
    thread::spawn(
        move
        ||{listen_for_servers(client_listener)}
    );
    thread::spawn(
        move
        ||{connect_to_main(mainaddr, server_listener_addr)}
    );

}

fn listen_for_clients(listener: TcpListener){}
fn listen_for_servers(listener: TcpListener){}

fn connect_to_main(mainaddr: SocketAddrV4, server_listener_addr: SocketAddrV4)-> Vec<SocketAddrV4>{
    let mut main_stream = TcpStream::connect(mainaddr).expect("could not connect to main server");
    let addr_json = serde_json::to_string(&server_listener_addr).unwrap();
    main_stream.write(addr_json.as_bytes());
    println!("server: {}",server_listener_addr);
    vec![]
}