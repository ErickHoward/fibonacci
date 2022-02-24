use clap::{Parser, Subcommand, ArgGroup};
use crate::fibonacci::get_n;

mod fibonacci;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]

struct Cli {
    #[clap(parse(try_from_str))]
    term: u128,

    #[clap(short, long)]
    all_terms: bool,

    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    output: bool,

    #[clap(long)]
    file_location: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.all_terms {
        println!("The first {} terms of the fibonacci sequence are: ", cli.term);

        let fibo_array = fibonacci::get_fibo_array(cli.term);
        println!("{:?}", fibo_array);
    } else {
        println!("The {}th term of the fibonacci sequence is: ", cli.term);
        println!("{:?}", fibonacci::fibo(cli.term));
    }

}