/*
    server tasks:
    liste af servere.
    listen for shares (add: number, shamir: number)
    listen for sums (add: sum, shamir: store in list based on server number (index in list)) 
    add shares to get sum
    share sum
    compute result () 
*/
use std::net::{TcpListener, TcpStream, Shutdown, SocketAddr};
use std::io::{Read, Write};
use num::rational::Ratio;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::Result;

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
    //server metoden får en strategy
    pub fn start_server(self, conns: Vec<String>, i: usize){

        //shares arc og sums arc
        let server_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let server_listener_addr = server_listener.local_addr().unwrap();
        let client_listener_addr = SocketAddr::new(server_listener_addr.ip(),server_listener_addr.port()+1);
        let client_listener = TcpListener::bind(client_listener_addr).unwrap();

        let ref_counter = Arc::new(Mutex::new(self));
        let ref_counter_clone_s = ref_counter.clone();
        let mut main_connection = TcpStream::connect("127.0.0.1:3333").unwrap();
        let addr_json = serde_json::to_string(&server_listener_addr).unwrap();
        main_connection.write(addr_json.as_bytes());
        //send adresse




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
        ref_counter.lock().unwrap().send.lock().unwrap().run_protocol(server_list);
        loop{}
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
