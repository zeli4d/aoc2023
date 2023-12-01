use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];

    println!("Input file: {}", input_path);


    let cont = fs::read_to_string(input_path)
        .expect("File not found");

    println!("{}", cont);

}
