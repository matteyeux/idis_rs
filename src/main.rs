use clap::Parser;
use idis_rs::IBoot;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 0)]
    count: u32,

    #[clap(short, long)]
    file: String,

    #[clap(short, long, default_value_t = 0)]
    skip: usize,
}

fn main() {
    let args = Args::parse();

    let mut iboot = IBoot::init(&args.file).unwrap_or_else(|err| {
        eprintln!("{} : {err}", args.file);
        process::exit(1);
    });

    if !iboot.is_iboot() {
        eprintln!("{} is not an iBoot", args.file);
        process::exit(1);
    }

    iboot.base_addr = iboot.get_base_addr();

    println!("Base address {:#02x?}", iboot.base_addr);

    iboot.disassemble(args.count, args.skip);
}
