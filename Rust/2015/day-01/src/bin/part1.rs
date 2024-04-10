fn main() {
    let input: &str = include_str!("./input1.txt");
    let output: i32 = process(input);
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

fn process(input: &str) -> i32 {
    let mut state = State::new("", 0);

    for c in input.chars() {
        state.data += &c.to_string();
        if c == '(' {
            state.floor += 1;
        }
        if c == ')' {
            state.floor -= 1;
        }
    }

    state.floor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(0, process("(())"));
    }
    #[test]
    fn test2() {
        assert_eq!(0, process("()()"));
    }
    #[test]
    fn test3() {
        assert_eq!(3, process("((("));
    }
    #[test]
    fn test4() {
        assert_eq!(3, process("(()(()("));
    }
    #[test]
    fn test5() {
        assert_eq!(3, process("))((((("));
    }
    #[test]
    fn test6() {
        assert_eq!(-1, process("())"));
    }
    #[test]
    fn test7() {
        assert_eq!(-3, process(")))"));
    }
    #[test]
    fn test8() {
        assert_eq!(-3, process(")())())"));
    }
}
