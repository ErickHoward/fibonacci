use std::fs::File;
use std::io::Write;

use clap::Parser;

mod fibonacci;
mod input;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]

struct Cli {
    #[clap(parse(try_from_str), help = "The term of the fibonacci sequence to output")]
    term: Option<u32>,

    #[clap(short, long, help = "Whether to print all terms from the 1st to the nth term as an array")]
    all_terms: bool,

    #[clap(short, long, help = "Write the output of the program to a file")]
    output: Option<String>,
}

fn main() {
	let cli = Cli::parse();

	if let Some(term) = cli.term {
		get_terms(term, cli.all_terms)
	} else {
		let (term, all_terms, output) = input::start_interactive_mode();
		get_terms(term, all_terms);
		match output {
			Some(dir) => {
				write_output(term, all_terms, dir);
			}
			None => (),
		}
	}

	if let Some(ref dir) = cli.output {
		write_output(cli.term.unwrap(), cli.all_terms, dir.to_string());
	}
}

fn get_terms(term: u32, all_terms: bool) {
	if all_terms {
		println!("The first {} terms of the fibonacci sequence are: ", term);

		let fibo_array = fibonacci::get_fibo_array(term);
		println!("{:?}", fibo_array);
	} else {
		println!("The {}th term of the fibonacci sequence is: ", term);
		println!("{:?}", fibonacci::fibo(term));
	}
}

fn write_output(term: u32, all_terms: bool, output_dir: String) {
	if all_terms {
		let fibo_array = fibonacci::get_fibo_array(term);
		let mut file = match File::create(output_dir) {
			Ok(file) => file,
			Err(error) => panic!("Problem creating file:\n{}", error),
		};
		match writeln!(&mut file, "{:?}", fibo_array) {
			Ok(_) => (),
			Err(error) => panic!("Problem writing to file:\n{}", error),
		}
	} else {
		let mut file = match File::create(output_dir) {
			Ok(file) => file,
			Err(error) => panic!("Problem creating file:\n{}", error),
		};

		match writeln!(&mut file, "{}", fibonacci::fibo(term)) {
			Ok(_) => (),
			Err(error) => panic!("Problem writing to file:\n{}", error),
		}
	}
}
