use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://adventofcode.com/2023/day/1

fn main() {
    let re = Regex::new(r"\d{1}").unwrap();
    // Filename must exist in the current path
    if let Ok(lines) = read_lines("./data/puzzle_1_data.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut line_num: i32 = 0;
        let mut sum: i32 = 0;
        for line in lines {
            println!("---\nLine {line_num}\n---");
            line_num += 1;
            if let Ok(ip) = line {
                let result: Vec<_> = re.find_iter(&ip).map(|m| m.as_str()).collect();
                let f = result.first().unwrap().to_owned();
                // last() will catch edge case where we only have a single value in our array
                let l = result.last().unwrap().to_owned();
                // Combine first/last as a string repr
                let number: i32 = [f, l].join("").parse().unwrap();
                println!(
                    "First int : {:?}, Last int : {:?}, combined value : {:?}\n---",
                    f, l, number
                );
                sum += number;
            }
        }
        println!("Final sum = {}", sum);
    }
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
