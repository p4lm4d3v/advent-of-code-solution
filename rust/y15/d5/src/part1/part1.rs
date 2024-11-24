mod test;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!("{}", output);
}

fn does_not_contain(input: &str) -> bool {
    let dont_contain = vec!["ab", "cd", "pq", "xy"];

    for dc in dont_contain.iter() {
        if input.contains(dc) {
            return false;
        }
    }

    true
}

fn letter_twice_in_a_row(input: &str) -> bool {
    let chars = input.chars().collect::<Vec<_>>();
    let pairs = chars.windows(2).collect::<Vec<_>>();

    let mut counter = 0;
    for pair in pairs.iter() {
        if pair[0] == pair[1] {
            counter += 1;
        }
    }
    counter >= 1
}

fn has_at_least_3_vowels(input: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let chars = input.chars().collect::<Vec<_>>();

    let mut counter = 0;
    for c in chars.iter() {
        if vowels.contains(&c) {
            counter += 1;
        }
    }

    counter >= 3
}

fn is_nice(input: &str) -> bool {
    has_at_least_3_vowels(input) && letter_twice_in_a_row(input) && does_not_contain(input)
}

fn process(input: &str) -> usize {
    let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

    let mut nice_counter = 0;
    for line in lines {
        if is_nice(line) {
            nice_counter += 1;
        }
    }

    nice_counter
}
