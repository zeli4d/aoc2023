use std::env;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    const RED: i32 = 12;
    const GREEN: i32 = 13;
    const BLUE: i32 = 14;

    let rx_gameid = Regex::new("^Game ([0-9]+): (.*)").unwrap();

    let mut cntr = 1;
    let mut output_no = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            cntr -= 1;
            // if cntr < 0 { break; };
            if let Ok(ip) = line {
                println!("{}\n", ip.as_str());

                let mut min_r = 0;
                let mut min_g = 0;
                let mut min_b = 0;


                let (full, [game_id, rounds]) = rx_gameid.captures(ip.as_str()).unwrap().extract();
                for round in rounds.split("; ") {
                    for col in round.split(", ") {
                        let (pcs, col) = col.split_once(" ").unwrap();
                        match col {
                            "red" => {
                                if pcs.parse::<i32>().unwrap() > min_r { min_r = pcs.parse::<i32>().unwrap() }
                            }
                            "green" => {
                                if pcs.parse::<i32>().unwrap() > min_g { min_g = pcs.parse::<i32>().unwrap() }
                            }
                            "blue" => {
                                if pcs.parse::<i32>().unwrap() > min_b { min_b = pcs.parse::<i32>().unwrap() }
                            }
                            _ => { println!("Unknown color {}", col) }
                        }
                    }
                }

                output_no += min_r*min_g*min_b;
            }
        }
    }

    println!("================================");
    println!("RESULT = {}", output_no)
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}