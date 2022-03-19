pub fn client(server_list: Vec<SocketAddrV4>, protocol: Protocol, prime: i64){
    let secret = create_secret();
    let shares = match protocol{
        Additive => {
            
        }
        //Shamir => {
        //    let coeffs = crate::shamir::create_secret(secret,server_list.len()-2,prime);
        //    crate::shamir::create_shares()
        //}
        _ => {
            println!{"enum does not exist"}
        }
    };
    // determine shares
    // send shares to servers
}

pub fn create_secret()-> i64{
    if rand::random(){ 1 } else { 0 }
}