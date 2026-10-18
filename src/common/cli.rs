use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short='i', long)]
    pub host: String,

    /// Number of times to greet
    #[arg(short='p', long,value_delimiter=',')]
    pub ports : Vec<u16>
}

pub fn receive() -> Args {
    let args = Args::parse();

    if args.host.trim().is_empty() {
        eprintln!("Host cannot be empty");
        std::process::exit(1);
    }

    if args.ports.is_empty(){
        eprintln!("Ports cannot be empty");
        std::process::exit(1);

    }

    args


    

}
