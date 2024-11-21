use clap::Parser;
use parallel_trial_division::{parallel_factorization, serial_factorization};
use std::time::Instant;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    number: u128,
    #[arg(long)]
    serial: bool,
}

fn main() {
    let cli = Cli::parse();

    let now = Instant::now();

    let factors = {
        if cli.serial {
            serial_factorization(cli.number)
        } else {
            parallel_factorization(cli.number)
        }
    };

    let elapsed_time = now.elapsed();
    println!("{:} = {:}", cli.number, factors);
    println!("Factored in {:} seconds", elapsed_time.as_secs_f64());
}
