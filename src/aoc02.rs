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

    let mut cntr = 5;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            cntr -= 1;
            if cntr < 0 { break };
            if let Ok(ip) = line {

                let (full, [game_id,rounds]) = rx_gameid.captures(ip.as_str()).unwrap().extract();
                // println!("G:'{}', '{}' - '{}'", game_id, rounds, ip.as_str());
                for draws in rounds.split("; ") {
                    // println!("{}", draws);
                    for col in draws.split(", ") {
                        let x = col.split_once(" ");
                        if x.is_some_and(x.) {  }
                        match col.split_once(" ") {
                            Some((n, v) v = RED) => {

                            },

                            _ => {}
                        }

                    }
                }

            }
        }
    }


}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}