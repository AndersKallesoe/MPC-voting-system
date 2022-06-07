
use crate::*;
use std::time::*;

fn check_results(result: i64, results: Vec<i64>)->bool{
    for r in results[1..].iter(){
        if *r != result{
            return false
        }
    };
    return true
}

/*
    Test
*/
pub fn test_additive(){
    let additive_protocol_1 = Protocol{prime: 29, servers: 2, voters: 20, protocol: ProtocolType::Additive};
    test_protocol(additive_protocol_1,vec![],100);   
    let additive_protocol_2 = Protocol{prime: 113, servers: 3, voters: 20, protocol: ProtocolType::Additive};
    test_protocol(additive_protocol_2,vec![],100);
    let additive_protocol_2 = Protocol{prime: 113, servers: 10, voters: 20, protocol: ProtocolType::Additive};
    test_protocol(additive_protocol_2,vec![],100);
    let additive_protocol_2 = Protocol{prime: 113, servers: 15, voters: 20, protocol: ProtocolType::Additive};
    test_protocol(additive_protocol_2,vec![],100);   
}
pub fn test_shamir(){
    let shamir_protocol_1 = Protocol{prime: 29, servers: 2, voters: 20, protocol: ProtocolType::Shamir};
    test_protocol(shamir_protocol_1,vec![],100);   
    let shamir_protocol_2 = Protocol{prime: 113, servers: 3, voters: 20, protocol: ProtocolType::Shamir};
    test_protocol(shamir_protocol_2,vec![],100);
    let shamir_protocol_2 = Protocol{prime: 113, servers: 10, voters: 20, protocol: ProtocolType::Shamir};
    test_protocol(shamir_protocol_2,vec![],100);
    let shamir_protocol_2 = Protocol{prime: 113, servers: 15, voters: 20, protocol: ProtocolType::Shamir};
    test_protocol(shamir_protocol_2,vec![],100);   
}
pub fn test_fault_detection(){
    let fd_protocol_1 = Protocol{prime: 29, servers: 3, voters: 20, protocol: ProtocolType::ShamirFaultDetection};
    test_protocol(fd_protocol_1,vec![1],100);   
    let fd_protocol_2 = Protocol{prime: 113, servers: 5, voters: 20, protocol: ProtocolType::ShamirFaultDetection};
    test_protocol(fd_protocol_2,vec![0,4],100);
    let fd_protocol_2 = Protocol{prime: 113, servers: 11, voters: 20, protocol: ProtocolType::ShamirFaultDetection};
    test_protocol(fd_protocol_2,vec![0,1,2,3,4],100);
    
}
pub fn test_error_correction(){
    let ec_protocol_1 = Protocol{prime: 29, servers: 4, voters: 20, protocol: ProtocolType::ShamirErrorCorrection};
    test_protocol(ec_protocol_1,vec![1],100);   
    let ec_protocol_2 = Protocol{prime: 113, servers: 7, voters: 20, protocol: ProtocolType::ShamirErrorCorrection};
    test_protocol(ec_protocol_2,vec![0,4],100);
    let ec_protocol_2 = Protocol{prime: 113, servers: 13, voters: 20, protocol: ProtocolType::ShamirErrorCorrection};
    test_protocol(ec_protocol_2,vec![0,1,2,3,12],100);
}

fn test_protocol(protocol: Protocol, corrupt: Vec<u8>, times: i64){
    line();
    println!("Testing protocol: {:?},s:{},v:{},p:{}",protocol.protocol,protocol.servers,protocol.voters,protocol.prime);
    println!("{} times:",times);
    let mut failure = 0;
    for _i in 0..times{
        let c = corrupt.clone();
        let ( mut result,results) = run_protocol(protocol,c);
        match protocol.protocol {
            ProtocolType::ShamirFaultDetection => {
                result = -1
            }
            _=> {}
        }
        if !check_results(result, results){
            failure = failure +1;
        }
    }
    if failure == 0{
        println!("all protocols ran without failure");
    }
    else{
        println!("the protocol failed {} times", failure);
    }
    line();
    println!();
}
/*
    Demonstrate
*/
pub fn demonstrate_additive(){
    let additive_protocol_1 = Protocol{prime: 29, servers: 2, voters: 20, protocol: ProtocolType::Additive};
    run_and_report(additive_protocol_1,vec![]);   
    //let additive_protocol_2 = Protocol{prime: 113, servers: 5, voters: 100, protocol: ProtocolType::Additive};
    //run_and_report(additive_protocol_2,vec![]);   
}
pub fn demonstrate_shamir(){
    let shamir_protocol_1 = Protocol{prime: 29, servers: 2, voters: 20, protocol: ProtocolType::Shamir};
    run_and_report(shamir_protocol_1,vec![]);
    //let shamir_protocol_2 = Protocol{prime: 113, servers: 5, voters: 10, protocol: ProtocolType::Shamir};
    //run_and_report(shamir_protocol_2,vec![]);
}
pub fn demonstrate_fault_detection(){
    let fault_detection_protocol_1 = Protocol{prime: 29, servers: 3, voters: 20, protocol: ProtocolType::ShamirFaultDetection};
    run_and_report(fault_detection_protocol_1,vec![1]);
    run_and_report(fault_detection_protocol_1,vec![]);
    let fault_detection_protocol_2 = Protocol{prime: 113, servers: 5, voters: 100, protocol: ProtocolType::ShamirFaultDetection};
    run_and_report(fault_detection_protocol_2,vec![1,4]);
    run_and_report(fault_detection_protocol_2,vec![]);
}
pub fn demonstrate_error_correction(){
    let error_correction_protocol_1 = Protocol{prime: 29, servers: 4, voters: 20, protocol: ProtocolType::ShamirErrorCorrection};
    run_and_report(error_correction_protocol_1, vec![1]);
    let error_correction_protocol_2 = Protocol{prime: 29, servers: 7, voters: 20, protocol: ProtocolType::ShamirErrorCorrection};
    run_and_report(error_correction_protocol_2, vec![1,6]);
    run_and_report(error_correction_protocol_2, vec![]);
    run_and_report(error_correction_protocol_2, vec![1]);
    run_and_report(error_correction_protocol_2, vec![1,4,6]);
}

fn run_and_report(protocol: Protocol, corruptions: Vec<u8>){
    let (result, results) = run_protocol(protocol, corruptions.clone());
    report_results(protocol, result, results, corruptions);
}

fn report_results(protocol: Protocol, result: i64, results: Vec<i64>, corruptions: Vec<u8>){
    line();
    println!("Results:");
    line();
    println!("Protocol: {:?}",protocol.protocol);
    println!("Servers: {}", protocol.servers);
    println!("Voters: {}", protocol.voters);
    println!("Prime: {}", protocol.prime);
    println!("Corruptions: {:?}", corruptions);
    line();
    println!("Actual Result: {}",result);
    println!("Server Results: {:?}", &results[1..]);
    line();
    let mut agree = true;
    let mut lastresult = results[1];
    for r in &results[1..] {
        if *r != lastresult{
            agree = false;
            break;
        }
        lastresult = *r;
    }
    if agree{
        println!("all servers agree");
         match results[1] {
            -1 => {println!("a fault was detected in the protocol")}
            -2 =>{println!("could not find polynomial consisting of integers!")}
            _=>{let success = check_results(result,results);
                println!("Protocol succes: {}",success);}
        };
        
    }else{
        println!("server disagree(there is a bug!)");
        println!("{:?}", results);
    }
    
    line();
}
fn line(){
    println!("________________________________________________________________________")
}

/*
    Benchmark
*/

pub fn benchmark_protocol(protocol: Protocol, corrupt: Vec<u8>, runs: i64){
    line();
    println!("Benchmarking protocol: {:?},s:{},v:{},p:{}",protocol.protocol,protocol.servers,protocol.voters,protocol.prime);
    println!("{} runs:",runs);
    let mut results = vec![];
    let mut failure = 0; 
    for _i in 0..runs{
        let now = Instant::now();
        let c = corrupt.clone();
        let (result,res) = run_protocol(protocol,c);
        if !check_results(result, res){
            failure = failure +1;
        }
        let elapsed_time = now.elapsed();
        results.push(elapsed_time);
    }
    println!("results: {:?}",results);
    println!("with {} failures", failure);
    line();
}


