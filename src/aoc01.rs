use std::env;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];

    println!("Input file: {}", input_path);

    let re = Regex::new(r"[0-9]").unwrap();
    // let mut results = vec![];

    let mut suma: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {

                // print!("{}", ip);

                let caps:Vec<_> = re.find_iter(ip.as_str()).map(|mat| mat.as_str()).collect();

                // print!(" {:?}", caps);
                suma += format!("{}{}", caps.first().unwrap(),  caps.last().unwrap()).parse::<i32>().unwrap();

                println!("{} = {}", ip, suma);
            }
        }
    }

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}