use std::net::*;
use std::time::*;
use crate::*;

pub fn client(server_list: Vec<(Ipv4Addr,u16)>, protocol: Protocol){
    thread::sleep(Duration::from_secs(2));
    let secret = create_secret();
    let shares = match protocol.protocol{
        ProtocolType::Additive => {
            additive::create_shares(secret, protocol.prime, protocol.servers as i64)
        }
        ProtocolType::Shamir => {
            let coefficients = shamir::create_coefficients(secret, (protocol.servers - 1) as i64, protocol.prime as u64);
            shamir::create_shares(&coefficients, protocol.servers as i64)
        }
        ProtocolType::ShamirFaultDetection => {
            let coefficients = shamir::create_coefficients(secret, shamir::detection_degree(protocol.servers), protocol.prime as u64);
            shamir::create_shares(&coefficients, protocol.servers as i64)
        }
        _ => {
            println!("pattern match failed");
            vec![]
        }
    };

    send_shares(server_list, secret, shares)
}

fn send_shares(server_list: Vec<(Ipv4Addr,u16)>, secret: i64, shares: Vec<i64>){
    connect_and_send(SocketAddrV4::new(server_list[0].0,server_list[0].1), secret);
    for i in 0..server_list.len()-1{
        let addrs = SocketAddrV4::new(server_list[i+1].0,server_list[i+1].1);
        connect_and_send(addrs, shares[i]);
    }
}

fn connect_and_send(addr: SocketAddrV4, n: i64){
    let mut error_string = String::from("could not connect to address ");
    error_string.push_str(&addr.to_string());
    let mut conn = TcpStream::connect(addr).expect(&error_string);
    let addr_json = serde_json::to_string(&n).unwrap();
    conn.write(addr_json.as_bytes()).expect("Error writing to server!");
}

fn create_secret()-> i64{
    if rand::random(){ 1 } else { 0 }
}