fn main() {
    let input: &str = include_str!("./input2.txt");
    let output: usize = process(input);
    dbg!("{}", output);
}

#[derive(Debug)]
struct State {
    data: String,
    floor: i32,
}

impl State {
    fn new(data: &str, floor: i32) -> Self {
        Self {
            data: data.to_string(),
            floor: floor,
        }
    }
}

fn process(input: &str) -> usize {
    let mut state = State::new("", 0);

    for c in input.chars() {
        state.data += &c.to_string();
        if c == '(' {
            state.floor += 1;
        }
        if c == ')' {
            state.floor -= 1;
        }
        if state.floor == -1 {
            break;
        }
    }

    state.data.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, process(")"));
    }
    #[test]
    fn test2() {
        assert_eq!(5, process("()())"));
    }
}
