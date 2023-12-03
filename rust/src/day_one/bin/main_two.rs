use aho_corasick::{AhoCorasick, MatchKind};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://adventofcode.com/2023/day/1
// If we had lookahaheads "\d{1}|(?=(one|two|three|four|five|six|seven|eight|nine))" would be fine

fn main() {
    let num_map: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    // We're making these patterns for the Aho Corasick Haystack parse; definitely a fancier way by
    // retrieving these from the num_map keys iter but I'm lazy
    // This is being done because regex crate doesn't have lookahead support :(
    let mut patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine","1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];
    // Filename must exist in the current path
    if let Ok(lines) = read_lines("./data/day_one/part_two_example_data.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut line_num: i32 = 0;
        let mut sum: i32 = 0;
        for line in lines {
            println!("---\nLine {line_num}\n---");
            line_num += 1;
            if let Ok(haystack) = line {
                let start_ac = AhoCorasick::new(patterns).unwrap();
                let mat_start = start_ac.find(&haystack).expect("should have a match");
                let mut first = &haystack[mat_start.start()..mat_start.end()];
                if first.len() != 1 {
                    first = num_map.get(first).unwrap();
                }

                let end_ac = AhoCorasick::builder()
                                    .match_kind(MatchKind::LeftmostFirst)
                                    .build(patterns)
                                    .unwrap();
                // last() will catch edge case where we only have a single value in our array
                let mat_last = end_ac.find(&haystack).expect("should have a match");
                let mut last = &haystack[mat_last.start()..mat_last.end()];
                //let mut last = result.last().unwrap().to_owned();
                if last.len() != 1 {
                    last = num_map.get(last).unwrap();
                }
                // Combine first/last as a string repr
                let number: i32 = [first, last].join("").parse().unwrap();
                println!(
                    "First int : {:?}, Last int : {:?}, combined value : {:?}\n---",
                    first, last, number
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
