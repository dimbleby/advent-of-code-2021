use structopt::StructOpt;
mod day01;

#[derive(StructOpt)]
struct Cli {
    day: u8,
}

fn main() {
    let args = Cli::from_args();
    match args.day {
        1 => day01::day01(),
        _ => println!("Unimplemented day: {}", args.day),
    }
}
