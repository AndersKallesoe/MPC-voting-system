/*
use std::{thread, time};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

struct AdditiveServer{
    conns: Mutex<Vec<u64>>,
    shares: Mutex<Vec<u64>>,
    sums: Mutex<Vec<u64>>
}
struct ShamirServer{
    conns: Mutex<Vec<u64>>,
    shares: Mutex<Vec<u64>>,
    sums: Mutex<Vec<(u64, u64)>> //shamir should be able to determine which sum of shares came from which server
}
struct SendServer{
    send: Arc<Mutex<dyn Server + Send>>
}
struct Shares{
    lock: Mutex<Vec<u64>>
}

/*
    server tasks:
    liste af servere.
    listen for shares (add: number, shamir: number)
    listen for sums (add: sum, shamir: store in list based on server number (index in list)) 
    add shares to get sum
    share sum
    compute result () 
*/





/*
    struct Server{
        Sharingstrategy: Sharingstrategy
        SharingStrategy.handleServer()
        conns: Mutex<Vec<u64>>,
        shares: Mutex<Vec<u64>>,
        sums: Mutex<Vec<(u64, u64)>> //shamir should be able to determine which sum of shares came from which server
    }
*/

/*
    factory metode som skaber tcp forbindelserne, gemmer ip:port og kalder startserver
*/
//fn server_factory(no_of_servers: u64, enum: (shamir or additive)){ returns }
/*
    let server_listener= TcpListener::bind("").unwrap();
    let client_listener= TcpListener::bind("").unwrap();
*/
impl SendServer{
    fn start_server(&'static mut self, server_list: &'static Vec<&str>, prime: u64){
        let server_listener= TcpListener::bind("").unwrap();
        let client_listener= TcpListener::bind("").unwrap();
        let shares = 
        Arc::new(
            Shares{
                lock:Mutex::new(vec![])
            });
        let shares_ref = shares.clone();
        thread::spawn(move|| self.listen_for_servers(shares_ref, server_listener));
    
        shares_ref = shares.clone();
        let server_clone = self.send.clone();
        thread::spawn(move|| {server_clone.lock().unwrap().run_protocol(server_list)});
        loop {}
    }

    fn listen_for_servers(&self, arc_ref: Arc<Shares>, listener: TcpListener){    
        for stream in listener.incoming(){
            let shares_ref_clone = arc_ref.clone();
            let server_clone = self.send.clone();
            match stream{
                Ok(stream) =>{
                    thread::spawn(
                        move || {
                            server_clone.lock().unwrap().handle_server(
                                stream, 
                            )
                        }
                    );
                }
                Err(_) => {println!("Error!"); panic!();}
            }
        }
    }
}

trait Server{
    fn handle_server(&self, stream: TcpStream);
    fn run_protocol(&self, server_list: &Vec<&str>);
}

impl Server for AdditiveServer{
    fn handle_server(&self, mut stream: TcpStream){
        let mut data = [0 as u8; 8];
        while match stream.read(&mut data){
            Ok(_size) => {
                //read share into shares struct
                let mut s = self.sums.lock().unwrap();
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
    fn run_protocol(&self, server_list: &Vec<&str>){
        wait();
        let sums = Arc::new(Mutex::new(vec![]));
        let sums_ref = sums.clone();
    }
}

impl Server for ShamirServer{
    fn handle_server(&self, mut stream: TcpStream){
        let mut data = [0 as u8; 8];
        while match stream.read(&mut data){
            Ok(_size) => {
                //read share into shares struct
                let mut s = self.sums.lock().unwrap();
                s.push({let mut d = [0 as u8; 8]; d[0] = data[0]; d[1] = data[1]; (d[0] as u64, d[1] as u64)});
                stream.write(&data).expect("Error");
                return
            }
            Err(_) => {
                println!("Error! 1");
                stream.write("Error occured. Try again!".as_bytes()).unwrap();
                stream.shutdown(Shutdown::Both).unwrap();
                false}
        }{}
    }
    fn run_protocol(&self, server_list: &Vec<&str>){
        wait();
        let sums = Arc::new(Mutex::new(vec![]));
        let sums_ref = sums.clone();
    }
}

fn listen_for_servers(arc_ref: Arc<Shares>, listener: TcpListener){    
    for stream in listener.incoming(){
        let shares_ref_clone = arc_ref.clone();
        match stream{
            Ok(stream) =>{
                thread::spawn(
                    move || {
                        handle_server(
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

fn listen_for_clients(arc_ref: Arc<Shares>, listener: TcpListener){
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
            false
        }
    }
    {}
}

fn handle_server(mut stream: TcpStream,sums: Arc<Sums>){
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

fn wait(){
    let sec = time::Duration::from_secs(1);
    thread::sleep(sec);
}
*/
/* 
fn sum_shares(){
    let mut sum = 0;
    for share in shares.lock.lock().unwrap().iter(){
        sum += share;
    }
 */
/*
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
}*/