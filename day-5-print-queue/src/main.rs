use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Lines},
};

use aoc_2024::read_input_by_line;

fn main() {
    if let Ok(mut lines) = read_input_by_line("input") {
        part_one(&mut lines);
    }
}

fn part_one(lines: &mut Lines<BufReader<File>>) {
    // rules map, key -> Vec of pages that must come before it
    let mut rules = HashMap::with_capacity(lines.size_hint().0);

    // load rules
    while let Some(Ok(line)) = lines.next() {
        // section separator
        let rule = line.trim();
        if rule.len() == 0 {
            // all rules loaded
            break;
        }

        // rule: x|y => if both x,y then x must occur before y
        // therefore if y, x must not occur after
        let (val, key) = rule.split_once('|').unwrap();

        // parse to usize to make everything nice and Copy
        let (val, key): (usize, usize) = (val.parse().unwrap(), key.parse().unwrap());
        rules
            .entry(key)
            .and_modify(|pages: &mut Vec<usize>| pages.push(val))
            .or_insert(Vec::from([val]));
    }

    let mut sum = 0;

    // for each update validate the pages using loaded rules
    while let Some(Ok(line)) = lines.next() {
        // parse the page numbers from the update
        let update = line
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect::<Vec<usize>>();

        // hashset to hold pages that must not appear after
        let mut disallowed = HashSet::new();
        let mut is_valid = true;
        for page in &update {
            // a disallowed page was found, set is_valid false and
            // go to next iteration
            if disallowed.contains(&page) {
                is_valid = false;
                break;
            }

            // add disallowed pages from the rules to the set
            if let Some(rules_pages) = rules.get(page) {
                for page in rules_pages {
                    disallowed.insert(page);
                }
            }
        }

        // if valid, add the middle page number to the sum
        if is_valid {
            sum += update[update.len() / 2];
        }
    }

    println!("Sum of middle page numbers: {}", sum);
}
