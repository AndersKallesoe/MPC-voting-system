use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex, MutexGuard};

struct Shares{
    shares: Mutex<Vec<u64>>,
}

pub fn start_server(addr: &'static str, voters: usize){
    let shares = Arc::new(Shares{shares: Mutex::new(Vec::new()),});
    
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming(){
        let shares_ref_clone = shares.clone();
        match stream{
            Ok(stream) =>{
                thread::spawn(
                    move || {
                        handle_connection(
                            addr,
                            stream, 
                            shares_ref_clone.shares.lock().unwrap()
                        )
                    }
                );
                if shares.shares.lock().unwrap().len() > voters{
                    break;
                }
            }
            Err(_) => {println!("Error!"); panic!();}
        }
    }
    let mut sum = 0;
    for share in shares.shares.lock().unwrap().iter(){
        sum += share;
    }
    println!("The sum of my shares is {}", sum)
}

pub fn handle_connection(addr: &str, mut stream: TcpStream, mut shares: MutexGuard<Vec<u64>>) {
    println!("Incoming connection!");
    let mut data = [0 as u8; 1];
    while match stream.read(&mut data){
        Ok(size) => {
            //read share into shares struct
            shares.push(u64::from_be_bytes({let mut d = [0 as u8; 8]; d[0] = data[0]; d}));
            println!("---> {}: Current shares: {:?}", addr,  shares);
            //stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!("Error! 1");
            stream.write("Error occured. Try again!".as_bytes()).unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
            false}
    }
    {}
}