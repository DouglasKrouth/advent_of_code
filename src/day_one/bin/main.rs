use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let re = Regex::new(r"\d{1}").unwrap();
    let mut sum: i32 = 0;
    // Filename must exist in the current path
    if let Ok(lines) = read_lines("./data/example_1_data.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut line_num: i32 = 0;
        for line in lines {
            println!("\n---\nLine {line_num}\n---");
            line_num += 1;
            if let Ok(ip) = line {
                let result: Vec<_> = re.find_iter(&ip).map(|m| m.as_str()).collect();
                let first = result.first().unwrap();
                let last = result.last().unwrap();
                println!("First int : {:?}\nLast int : {:?} ", first, last);
            }
        }
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
