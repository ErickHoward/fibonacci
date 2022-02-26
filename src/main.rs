use clap::{ArgGroup, Parser, Subcommand};
use std::env;
use std::fs::{copy, File};
use std::io::Write;

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
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(dir) = cli.output.as_deref() {
        write_output(&cli, dir.to_string());
    }

    get_terms(&cli);
}

fn get_terms(cli: &Cli) {
    if cli.all_terms {
        println!(
            "The first {} terms of the fibonacci sequence are: ",
            cli.term
        );

        let fibo_array = fibonacci::get_fibo_array(cli.term);
        println!("{:?}", fibo_array);
    } else {
        println!("The {}th term of the fibonacci sequence is: ", cli.term);
        println!("{:?}", fibonacci::fibo(cli.term));
    }
}

fn write_output(cli: &Cli, output_dir: String) {
    if cli.all_terms {
        let fibo_array = fibonacci::get_fibo_array(cli.term);
        let mut file = match File::create(output_dir) {
            Ok(file) => file,
            Err(error) => panic!("Problem creating file:\n{}", error)
        };
        match writeln!(&mut file, "{:?}", fibo_array) {
            Ok(_) => (),
            Err(error) => panic!("Problem writing to file:\n{}", error)
        }
    } else {
        let mut file = match File::create(output_dir) {
            Ok(file) => file,
            Err(error) => panic!("Problem creating file:\n{}", error)
        };

        match writeln!(&mut file, "{}", fibonacci::fibo(cli.term)) {
            Ok(_) => (),
            Err(error) => panic!("Problem writing to file:\n{}", error)
        }
    }
}
