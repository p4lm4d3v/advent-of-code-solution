mod test1;

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output: i32 = process(input);
    dbg!("{}", output);
}

fn process(input: &str) -> i32 {
    let mut floor: i32 = 0;

    for c in input.chars() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }
    }

    floor
}
