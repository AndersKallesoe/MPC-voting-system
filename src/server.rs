use std::{thread, time};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};


struct Shares{
    lock: Mutex<Vec<u64>>,
}

struct Sums{
    lock: Mutex<Vec<u64>>,
}

pub fn start_server(addr: [&'static str; 2], index: usize, prime: u64){


    //listen for voters
    let shares = Arc::new(Shares{lock: Mutex::new(Vec::new()),});
    let shares_ref_clone = shares.clone();
    thread::spawn(move|| listen(shares_ref_clone, addr[index]));
    
    //sleep
    let sec = time::Duration::from_secs(1);
    thread::sleep(sec);
    //ad shares together
    let mut sum = 0;
    for share in shares.lock.lock().unwrap().iter(){
        sum += share;
    }
    println!("The sum of my shares is {}", sum);
    let sums = Arc::new(Sums{lock: Mutex::new(vec![sum])});
    let sums_ref_clone = sums.clone();
    //connect to other servers
    thread::spawn(move ||connect_to_servers(["127.0.0.1:3334", "127.0.0.1:3336"], index ,sums_ref_clone));
    thread::sleep(sec);
    let mut votes = 0;
    for sum in sums.lock.lock().unwrap().iter(){
        votes += sum;
    }
    votes = votes % prime;
    print!("i am server [{}] and ", addr[index]);
    println!("the result of the vote is: {}", votes);
}

fn connect_to_servers(addrs: [&'static str; 2], index: usize, sums_ref: Arc<Sums>){
    
    if index == 0{
        let listener = TcpListener::bind(addrs[index]).unwrap();
        for stream in listener.incoming(){
            match stream{
                Ok(stream) =>{
                    let sums_ref_clone = sums_ref.clone();
                    thread::spawn(move ||handle_server(stream, sums_ref_clone));
                }
                Err(_) => {println!("Error!"); panic!();}
            }
        }
    }else{
        let sums_ref_clone = sums_ref.clone();
        connect_and_share(addrs[0],sums_ref_clone)
    }
}

fn connect_and_share(addr: &str, sums: Arc<Sums>){
    let mut stream = TcpStream::connect(addr).expect("Error connecting to server");
    let mut s = sums.lock.lock().unwrap();
    stream.write(&s[0].to_le_bytes()).expect("Error");
    let mut data = [0 as u8; 8];
    while match stream.read(&mut data){
        Ok(_size) => {
            s.push(u64::from_le_bytes({let mut d = [0 as u8; 8]; d[0] = data[0]; d}));
            return
        }
        Err(_) => {
            println!("Error! 1");
            stream.write("Error occured. Try again!".as_bytes()).unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
            false}
    }{}
    
}


fn handle_server( mut stream: TcpStream,sums: Arc<Sums>){
    let mut data = [0 as u8; 8];
    while match stream.read(&mut data){
        Ok(_size) => {
            //read share into shares struct
            let mut s = sums.lock.lock().unwrap();
            s.push(u64::from_le_bytes({let mut d = [0 as u8; 8]; d[0] = data[0]; d}));
            stream.write(&s[0].to_le_bytes()).expect("Error");
            return
        }
        Err(_) => {
            println!("Error! 1");
            stream.write("Error occured. Try again!".as_bytes()).unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
            false}
    }{}
}


fn listen(arc_ref: Arc<Shares>, addr: &'static str){
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming(){
        let shares_ref_clone = arc_ref.clone();
        match stream{
            Ok(stream) =>{
                thread::spawn(
                    move || {
                        handle_client(
                            stream, 
                            shares_ref_clone
                        )
                    }
                );
            }
            Err(_) => {println!("Error!"); panic!();}
        }
    }
}

fn handle_client(mut stream: TcpStream, shares: Arc<Shares>) {
    let mut data = [0 as u8; 8];
    while match stream.read(&mut data){
        Ok(_size) => {
            let mut s = shares.lock.lock().unwrap();
            s.push(u64::from_le_bytes({let mut d = [0 as u8; 8]; d[0] = data[0]; d}));
            return
        }
        Err(_) => {
            println!("Error! 1");
            stream.write("Error occured. Try again!".as_bytes()).unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
            false}
    }{}
}