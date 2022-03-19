use std::net::*;
use crate::*;

pub fn client(server_list: Vec<(Ipv4Addr,u16)>, protocol: Protocol){
    let secret = create_secret();
    
    let shares = match protocol{
        Protocol::Additive{prime} => {
            additive::create_shares(secret, prime, (server_list.len()-1) as i64)
        }
        Protocol::Shamir{prime} => {
           shamir::create_shares(secret,prime,(server_list.len()-1) as i64)
        }
        _ => {
            println!("pattern match failed");
            vec![]
        }
    };

    send_shares(server_list, secret, shares)
}

fn send_shares(server_list: Vec<(Ipv4Addr,u16)>, secret: i64, shares: Vec<i64>){
    connect_and_send(SocketAddrV4::new(server_list[0].0,server_list[0].1+1), secret);
    for i in 0..server_list.len()-1{
        let addrs = SocketAddrV4::new(server_list[i+1].0,server_list[i+1].1+1);
        connect_and_send(addrs, shares[i]);
    }
}

fn connect_and_send(addr: SocketAddrV4, n: i64){
    let mut conn = TcpStream::connect(addr).expect("could not connect to main server");
    let addr_json = serde_json::to_string(&n).unwrap();
    conn.write(addr_json.as_bytes()).expect("Error writing to server!");
}

fn create_secret()-> i64{
    if rand::random(){ 1 } else { 0 }
}