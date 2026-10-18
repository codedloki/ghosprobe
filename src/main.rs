mod common;
mod scanner;
use crate::common::target::Target;
use crate::common::cli;


use crate::scanner::tcp;
fn main() {
    println!("Hello, world!");
    let args = cli::receive();

    println!("Args receive : {:?}",&args);

    let my_target = match Target::new(args.host,args.ports){
        Ok(target) => target,
        Err(e) => {
            eprintln!("{}",e);
            std::process::exit(1);
        }

    };
    let status  = tcp::scan_target(&my_target);

    for result in &status{
        if result.is_open {
            println!("Port {} is open",result.port);
        }
        else{
            println!("Port {} is closed",result.port);
        }
    }



    //println!("{}",target);

}
