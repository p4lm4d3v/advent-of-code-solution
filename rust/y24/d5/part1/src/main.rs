extern crate d5;

use d5::{follows_rules, parser, typedef};

use follows_rules::follows_rules;
use parser::Parser;
use typedef::Updates;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!("{}", output);
}

fn process(input: &str) -> i32 {
    let (rules, updates) = Parser::parse(input);

    let valid_updates: Updates = updates
        .iter()
        .filter(|&u| follows_rules(u, &rules))
        .map(|u| u.clone())
        .collect();

    let middles = valid_updates
        .iter()
        .map(|u| u[u.len() / 2])
        .collect::<Vec<i32>>();

    middles.iter().sum()
}
