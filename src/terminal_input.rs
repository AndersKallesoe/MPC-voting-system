use crate::*;
use std::io;

pub fn run_system(){
    println!("0 -> Run specific protocol, 1 -> demonstrate specific protocol, 2 -> test specific protocol");
    let mut run = String::new();
    io::stdin()
        .read_line(&mut run)
        .expect("Failed to read input");
    match run.trim().parse::<u32>(){
        Ok(i) =>
        match i {
            0 => {run_protocol_with_input()},
            1 => {run_demonstrate_protocol()},
            2 => {run_test_protocol()},
            _ => {run_system()}
        }
        Err(_) => {
            run_system()
        }
    }
    run_system()
}

fn run_demonstrate_protocol(){
	println!("Select protocol: 1-> Additive, 2-> Shamir, 3-> Fault Detection, 4 -> Error Correction");
	let mut protocol = String::new();
	io::stdin()
		.read_line(&mut protocol)
		.expect("Failed to read input");
    match protocol.trim().parse::<u8>(){
        Ok(i) => match i{
            1 => {test::_demonstrate_additive(); run_system()},
            2 => {test::_demonstrate_shamir(); run_system()},
            3 => {test::_demonstrate_fault_detection(); run_system()},
            4 => {test::_demonstrate_error_correction(); run_system()},
            _ => {println!("Please enter an integer in [1, 2, 3, 4]"); run_demonstrate_protocol()}
        },
        Err(e) => {
            {println!("Please enter an integer in [1, 2, 3, 4] ERROR"); println!("str: {:?}", e); run_demonstrate_protocol()}
        }
    }
}

fn run_test_protocol(){
	println!("Select protocol: 1-> Additive, 2-> Shamir, 3-> Fault Detection, 4 -> Error Correction");
	let mut protocol = String::new();
	io::stdin()
		.read_line(&mut protocol)
		.expect("Failed to read input");
    match protocol.trim().parse::<u8>(){
        Ok(i) => match i{
            1 => {test::_test_additive(); run_system()},
            2 => {test::_test_shamir(); run_system()},
            3 => {test::_test_fault_detection(); run_system()},
            4 => {test::_test_error_correction(); run_system()},
            _ => {println!("Please enter an integer in [1, 2, 3, 4]"); run_test_protocol()}
        },
        Err(e) => {
            {println!("Please enter an integer in [1, 2, 3, 4] ERROR"); println!("str: {:?}", e); run_test_protocol()}
        }
    }
}

fn run_protocol_with_input(){
	let protocoltype = get_protocol_with_input();
	let servers = get_servers_with_input();
    let prime = get_prime_with_input(servers.clone());
    let voters = get_clients_with_input();
	let corruptions = get_corruptions_with_input(servers.clone());
    let protocol = Protocol{prime,servers,voters,protocol:protocoltype};
    let print = get_print_with_input();
    crate::test::run_and_report(protocol,corruptions, print);
    run_system()
}

fn get_print_with_input() -> bool{
    println!("Print Client and server intermediate results? 0 -> no, 1 -> yes");
    let mut print = String::new();
    io::stdin()
        .read_line(&mut print)
        .expect("Failed to read input");
    match print.trim().parse::<u8>(){
        Ok(i) =>{
            match i {
                0 => {false},
                1 => {true},
                _ => {get_print_with_input()}
            }
        }
        Err(_)=>{get_print_with_input()}
    }
}

fn get_protocol_with_input()->ProtocolType{
	println!("Select protocol: 1-> Additive, 2-> Shamir, 3-> Fault Detection, 4 -> Error Correction");
	let mut protocol = String::new();
	io::stdin()
		.read_line(&mut protocol)
		.expect("Failed to read input");
    match protocol.trim().parse::<u8>(){
        Ok(i) => match i{
            1 => {ProtocolType::Additive},
            2 => {ProtocolType::Shamir},
            3 => {ProtocolType::ShamirFaultDetection},
            4 => {ProtocolType::ShamirErrorCorrection},
            _ => {println!("Please enter an integer in [1, 2, 3, 4]"); get_protocol_with_input()}
        },
        Err(e) => {
            {println!("Please enter an integer in [1, 2, 3, 4] ERROR"); println!("str: {:?}", e); get_protocol_with_input()}
        }
    }
}

fn get_servers_with_input()->u8{
	println!("Select number of servers (more than 15 may cause problems with ports): ");
	let mut servers = String::new();
	io::stdin()
		.read_line(&mut servers)
		.expect("Failed to read input");
	match servers.trim().parse::<u8>(){
        Ok(i) => {println!();println!("Input accepted!"); i}
        Err(_) => {println!();println!("Please provide a positive integer");get_servers_with_input()}
    }
}

fn get_prime_with_input(servers: u8)->i64{
	println!("Select prime (prime must be larger than servers): ");
	let mut prime = String::new();
	io::stdin()
		.read_line(&mut prime)
		.expect("Failed to read input");
    match prime.trim().parse::<i64>(){
        Ok(i) => {println!();println!("Input accepted!"); i}
        Err(_) => {println!();println!("Please provide a positive integer");get_prime_with_input(servers)}
    }
}

fn get_clients_with_input()->u16{
	println!("Select number of clients:");
	let mut clients = String::new();
	io::stdin()
		.read_line(&mut clients)
		.expect("Failed to read input");
    match clients.trim().parse::<u16>(){
        Ok(i) => {println!();println!("Input accepted!"); i}
        Err(_) => {println!();println!("Please provide a positive integer");get_clients_with_input()}
    } 	
}

fn get_corruptions_with_input(servers: u8)->Vec<u8>{
	println!("Select corrupted servers (for example \"1\" or \"2\" or \"8\"). Enter -1 to lock in:");
	let mut corruptions = vec![];
    let mut cont = true;
    while cont{
        println!("Next: ");
        let mut corrupt = String::new();
        io::stdin()
            .read_line(&mut corrupt)
            .expect("Failed to read input");
            match corrupt.trim().parse::<i32>(){
                Ok(i) => {
                    match i{
                        -1 => {
                            cont = false
                        }
                        _ =>{
                            if i > (servers as i32){
                                println!();println!("Please provide a positive integer smaller than {} or -1 to finish!", servers);
                            }
                            else if corruptions.contains(&(i as u8)){
                                println!();println!("That server was already picked to be corrupt!");
                            }
                            else{
                                println!();println!("Input accepted!"); corruptions.push(i as u8)
                            }
                        }
                    }
                }
                Err(_) => {println!();println!("Please provide a positive integer or -1 to finish!")}
            }
    };
    corruptions.sort();
    corruptions
}

