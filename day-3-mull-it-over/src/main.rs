use aoc_2024::read_input_to_string;
use regex::{Captures, Regex};

fn main() {
    let mut buf = String::with_capacity(20 * 1000);
    if let Ok(_len) = read_input_to_string("input", &mut buf) {}

    part_one(&buf);
    part_two(&buf);
}

fn part_one(input: &String) {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to compile regex");
    let captures = regex.captures_iter(input);
    let reduction = |acc, c: Captures| {
        acc + c.get(1).unwrap().as_str().parse::<usize>().unwrap()
            * c.get(2).unwrap().as_str().parse::<usize>().unwrap()
    };
    let result = captures.fold(0, reduction);
    println!("Part One Sum: {}", result);
}

fn part_two(input: &String) {
    let do_regex = Regex::new(r"do\(\)").expect("Failed to compile the 'do' regex");
    let dont_regex = Regex::new(r"don't\(\)").expect("Failed to compile the 'dont' regex");
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to compile 'mul' regex");

    let mut start = 0;
    let mut result = 0;
    while start < input.len() - 1 {
        // find the next don't() and set end
        let end = if let Some(next_dont) = dont_regex.find_at(input, start) {
            next_dont.end()
        } else {
            input.len() - 1
        };
        // process all instructions before the don't()
        while start < end {
            if let Some(captures) = mul_regex.captures_at(input, start) {
                let whole_match = captures.get(0).unwrap();
                // if a mul instruction is matched before the don't
                // process the instruction
                if whole_match.start() < end {
                    start = whole_match.end();
                    result += captures.get(1).unwrap().as_str().parse::<usize>().unwrap()
                        * captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
                } else {
                    // otherwise find the next do() and set start
                    start = if let Some(next_do) = do_regex.find_at(input, start) {
                        next_do.end()
                    } else {
                        input.len() - 1
                    }
                }
            } else {
                // no more instructions to process
                start = input.len();
            }
        }
    }
    println!("Part Two Sum: {}", result);
}
