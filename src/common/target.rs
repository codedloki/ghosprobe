use std::net::Ipv4Addr;
use std::str::FromStr;

pub struct Target{
    pub host:String,
    pub ports : Vec<u16>
}

impl Target{
    pub fn new(host:String,ports:Vec<u16>) -> Result<Self,String> {
        if Ipv4Addr::from_str(&host).is_err(){
                        return Err(format!("❌ Invalid IPv4 address: '{}'. Please provide a valid IP.", host));

        }
        Ok(Self{
            host,
            ports
        })
    }
}

#[derive(Debug)]
pub struct ScanResult {
    pub port:u16,
    pub is_open:bool,
}
