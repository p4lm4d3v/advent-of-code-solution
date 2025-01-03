extern crate d5;

use d5::parser::Parser;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!("{}", output);
}

fn process(input: &str) -> i32 {
    let (rules, updates) = Parser::parse(input);

    // let invalid_updates: Updates = updates
    //     .iter()
    //     .filter(|&u| !follows_rules(u, &rules))
    //     .map(|u| u.clone())
    //     .collect();

    // let sorted: Updates = invalid_updates
    //     .iter()
    //     .map(|u| sort_by_rules(u, &rules).clone())
    //     .collect();

    // let middles = sorted.iter().map(|u| u[u.len() / 2]).collect::<Vec<i32>>();

    // middles.iter().sum()

    todo!()
}
