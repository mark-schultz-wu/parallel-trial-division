use clap::Parser;
use parallel_trial_division::{parallel_factorization, serial_factorization};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    number: u128,
    #[arg(long)]
    serial: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.serial {
        println!("{:?}", serial_factorization(cli.number));
    } else {
        println!("{:?}", parallel_factorization(cli.number));
    }
}
