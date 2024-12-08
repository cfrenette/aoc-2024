use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Lines},
};

use aoc_2024::read_input_by_line;

fn main() {
    if let Ok(mut lines) = read_input_by_line("input") {
        let rules = load_rules(&mut lines);

        let mut sum = 0;
        let mut not_sorted_sum = 0;

        // for each update validate the pages using loaded rules
        while let Some(Ok(line)) = lines.next() {
            // parse the page numbers from the update
            let mut update = line
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
            } else {
                sort(&mut update, &rules);
                not_sorted_sum += update[update.len() / 2];
            }
        }

        println!("Sum of already sorted middle page numbers: {}", sum);
        println!(
            "Sum of not-sorted middle page numbers once sorted: {}",
            not_sorted_sum
        )
    }
}

fn load_rules(lines: &mut Lines<BufReader<File>>) -> HashMap<usize, Vec<usize>> {
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

    rules
}

fn sort(update: &mut Vec<usize>, rules: &HashMap<usize, Vec<usize>>) {
    // topological sort (backwards edition)
    // TODO: clean this up

    // create an adjacency list
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); update.len()];
    let mut pages_in_update = HashMap::with_capacity(update.len());

    // map each value in update to its index
    for (j, page) in update.iter().enumerate() {
        pages_in_update.insert(page, j);
    }

    // only add edges from rules for pages that are in the update
    for (i, page) in update.iter().enumerate() {
        if let Some(neighbors) = rules.get(page) {
            for neighbor in neighbors {
                if let Some(j) = pages_in_update.get(neighbor) {
                    adj[i].push(*j);
                }
            }
        }
    }

    // actually do the topo sort
    let mut visited = vec![false; update.len()];
    let mut sorted = Vec::with_capacity(update.len());
    for i in 0..update.len() {
        if !visited[i] {
            dfs(update, &adj, &mut visited, &mut sorted, i);
        }
    }
    for i in 0..sorted.len() {
        update[i] = sorted[i];
    }
}

fn dfs(
    update: &mut Vec<usize>,
    adj: &Vec<Vec<usize>>,
    visited: &mut [bool],
    sorted: &mut Vec<usize>,
    i: usize,
) {
    visited[i] = true;
    let neighbors = &adj[i];
    for neighbor in neighbors {
        if !visited[*neighbor] {
            dfs(update, adj, visited, sorted, *neighbor)
        }
    }
    sorted.push(update[i]);
}
