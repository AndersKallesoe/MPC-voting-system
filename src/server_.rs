/*
    server tasks:
    liste af servere.
    listen for shares (add: number, shamir: number)
    listen for sums (add: sum, shamir: store in list based on server number (index in list)) 
    add shares to get sum
    share sum
    compute result () 
*/
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use num::rational::Ratio;
use std::{thread, time};
use std::sync::{Arc, Mutex};

pub struct SendServer{
    pub send: Arc<Mutex<dyn Server + Send>>
    //sharing strategy to implement handle server?
}

pub struct Additive;
pub struct Shamir{
    shares: Vec<Ratio<i64>>,
    sums: Vec<Ratio<i64>>
}
pub trait Server{
    fn handle_server(&self, conn: TcpStream);
    fn handle_client(&self, conn: TcpStream);
    fn run_protocol(&self, conns: Vec<TcpStream>) -> Ratio<i64>;
}
impl SendServer{
    pub fn start_server(self, conns: Vec<String>, i: usize){
        let server_listener = SendServer::listen_available_port().unwrap();
        let client_listener = SendServer::listen_available_port().unwrap();
        let ref_counter = Arc::new(Mutex::new(self));
        let ref_counter_clone_s = ref_counter.clone();
        thread::spawn(
            move||{
                ref_counter_clone_s.lock().unwrap().listen_for_servers(server_listener)
            }
        );
        let ref_counter_clone_c = ref_counter.clone();
        thread::spawn(
            move||{
                ref_counter_clone_c.lock().unwrap().listen_for_clients(client_listener)
            }
        );
        //connect to servers based on vector of connections and index telling you which one you are
        let server_list = self.connect_to_servers(&conns, i);
        //wait
        let ref_counter_clone_p = ref_counter.clone();
        ref_counter_clone_p.lock().unwrap().send.lock().unwrap().run_protocol(server_list);
        loop{}
    }

    fn listen_available_port() -> Option<TcpListener> {
        for port in 1025..65535 {
            match TcpListener::bind(("127.0.0.1", port)) {
                Ok(listener) => return Some(listener),
                _ => ()
            }
        }
        None
    }

    fn connect_to_servers(&self, conns: &Vec<String>, i: usize) -> Vec<TcpStream>{
        let conn_vec = vec![];
        for (j, addr) in conns.iter().enumerate(){
            if j == i {continue}
            let mut stream = TcpStream::connect(addr);
            match stream{
                Ok(stream) => {conn_vec.push(stream)}
                Err(_) => {println!("Error connecting to {}", addr);}
            }
        }
        return conn_vec
    }

    fn listen_for_clients(&self, l: TcpListener){
        for stream in l.incoming(){
            let server_clone = self.send.clone();
            match stream{
                Ok(stream)=>{
                    thread::spawn(
                        move||{
                            server_clone.lock().unwrap().handle_client(stream);
                        }
                    );
                }
                Err(_) => {println!("Error in connection from client!")}
            }
        }
    }

    fn listen_for_servers(&self, l: TcpListener){
        for stream in l.incoming(){
            let server_clone = self.send.clone();
            match stream{
                Ok(stream)=>{
                    thread::spawn(
                        move||{
                            server_clone.lock().unwrap().handle_client(stream);
                        }
                    );
                }
                Err(_)=>{println!("Error in connection from server!")}
            }
        }
    }
}

impl Server for Additive{
    fn handle_server(&self, conn: TcpStream){

    }
    fn handle_client(&self, conn: TcpStream){

    }
    fn run_protocol(&self, conns: Vec<TcpStream>) -> Ratio<i64>{
        Ratio::new(1, 1)
    }
}

impl Server for Shamir{
    fn handle_server(&self, conn: TcpStream){

    }
    fn handle_client(&self, conn: TcpStream){
        let mut data = [0 as u8; 8];
        match conn.read(&mut data){
            Ok(size)=>{
                let share = Ratio::new(i64::from_le_bytes(data), 1);
                self.shares.push(share);
            }
            Err(_)=>{}
        }
    }
    fn run_protocol(&self, conns: Vec<TcpStream>) -> Ratio<i64>{

        Ratio::new(1, 1)
    }
}
