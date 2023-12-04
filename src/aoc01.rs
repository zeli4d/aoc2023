use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use fancy_regex::{Regex};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    let number = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);


    // println!("Input file: {}", input_path);

    let re = Regex::new(r"(?=([0-9]|zero|one|two|three|four|five|six|seven|eight|nine))").unwrap();

    let mut suma: i32 = 0;
    let mut num: i32;


    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(ip) = line {
                let mut all_captures = re.captures_iter(ip.as_str());


                let num_str = match all_captures.next() {
                    None => continue,
                    Some(m) => m.unwrap().get(1).unwrap().as_str()
                };


                let mut first = num_str;

                let mut last = match all_captures.last() {
                    None => "",
                    Some(m) => m.unwrap().get(1).unwrap().as_str()
                };


                if last == "" {
                    last = first;
                    // first = "0";
                }

                num = format!("{}{}", number.get(first).unwrap(), number.get(last).unwrap()).parse::<i32>().unwrap();
                suma += num;

                println!("{} {}, {} = {}", num, first, last, ip);
            }
        }
    }

    println!("celkem: {}", suma);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}