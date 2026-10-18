use std::net::{TcpStream};
use std::time::Duration;
use crate::common::target::{Target,ScanResult};

pub fn scan_target(target:&Target)->Vec<ScanResult>{
    let mut result = Vec::new();
    let timeout = Duration::from_millis(500);
    println!("Scanning {} on {} ports",target.host,target.ports.len());
    for &port in &target.ports {
        let add_str = format!("{}:{}",target.host,port);
        let addr = add_str.parse().expect("Failed to parse address");
        let is_open = TcpStream::connect_timeout(&addr,timeout).is_ok();
        println!("Status : {}",is_open);
        result.push(ScanResult{port,is_open})
    }
    result

}
