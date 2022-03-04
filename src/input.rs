use std::io;

fn get_n() -> u32 {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input.");

    let input: u32 = input
        .trim()
        .parse()
        .expect("Could not parse number.");

    input
}

fn get_wants_all_terms() -> bool {
    println!("Would you like all of the terms from 1 to the number you entered?");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input.");

    let input: bool = match input.trim() {
        "Yes" | "yes" | "Y" | "y" => true,
        "No" | "no" | "N" | "n" => false,
        _ => {
            println!("That is not a valid answer.");
            get_wants_all_terms()
        }
    };

    input
}

fn get_wants_output() -> bool {
    println!("Would you like to output the results to a file?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input.");

    match input.trim() {
        "Yes" | "yes" | "Y" | "y" => true,
        "No" | "no" | "N" | "n" => false,
        _ => {
            println!("That is not a valid answer.");
            get_wants_output()
        }
    }
}

fn get_output_dir() -> String {
    println!("Where would you like to save the output?");

    let mut input: String = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(error) => panic!("Problem reading input:\n{}", error),
    }

    input.trim().to_string()
}


pub fn start_interactive_mode() -> (u32, bool, Option<String>) {
    let term = get_n();
    let all_terms = get_wants_all_terms();
    let wants_output = get_wants_output();
    if wants_output {
        let output_file = get_output_dir();
        (term, all_terms, Some(output_file))
    } else {
        (term, all_terms, None)
    }
}
