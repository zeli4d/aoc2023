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
            // if cntr < 0 { break };
            if let Ok(ip) = line {
                println!("{}\n", ip.as_str());
                let mut game_pass = 0;

                let (full, [game_id,rounds]) = rx_gameid.captures(ip.as_str()).unwrap().extract();
                println!("G:{}", game_id);
                println!("Rounds:{}", rounds);
                for round in rounds.split("; ") {
                    println!("Round: {}", round);
                    for col in round.split(", ") {
                        println!("Color: {}", col);
                        let (pcs, col) = col.split_once(" ").unwrap();
                        println!("{} = {}", col, pcs);
                        match col {
                            "red" => { if pcs.parse::<i32>().unwrap() > RED { game_pass += 1 }},
                            "green" => { if pcs.parse::<i32>().unwrap() > GREEN { game_pass += 1 }},
                            "blue" => { if pcs.parse::<i32>().unwrap() > BLUE { game_pass += 1 }},
                            _ => {println!("Unknown color {}", col)}
                        }
                    }
                }

                if game_pass == 0 {
                    output_no += game_id.parse::<i32>().unwrap();
                }

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