fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn md5(input: &str) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

fn process(input: &str) -> usize {
    let mut i = input.to_string();
    let mut hash = md5(&i);

    let mut number = 0;
    while !hash.starts_with("00000") {
        number += 1;
        i = input.to_string() + number.to_string().as_str();
        hash = md5(&i);
    }
    println!("{} => {}", i, &hash);

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = process("abcdef");
        assert_eq!(result, 609043);
    }
    #[test]
    fn part1_test2() {
        let result = process("pqrstuv");
        assert_eq!(result, 1048970);
    }
    // #[test]
    // fn part1_test3() {
    //     let result = process("abcdef609043");
    //     assert_eq!(result, 609043);
    // }
    // #[test]
    // fn part1_test4() {
    //     let result = process("pqrstuv1048970");
    //     assert_eq!(result, 1048970);
    // }
}
