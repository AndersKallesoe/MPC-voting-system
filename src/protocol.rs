// trait
trait Protocol{
    fn run_protocol(&self, conns: Vec<TcpStream>) -> Ratio<i64>
}

impl Protocol for Additive{
    fn run_protocol(&self, conns: Vec<TcpStream>) -> Ratio<i64>{
        Ratio::new(1, 1)
    }
}

impl Protocol for Shamir{
    fn run_protocol(&self, conns: Vec<TcpStream>) -> Ratio<i64>{

        Ratio::new(1, 1)
    }
}