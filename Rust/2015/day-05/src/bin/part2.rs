fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn two_letter_pair(input: &str) -> (bool, String) {
    let chars = input.chars().enumerate().collect::<Vec<_>>();

    let pairs = chars
        .windows(2)
        .enumerate()
        // .map(|pair| pair[0].to_string() + pair[1].to_string().as_str())
        .collect::<Vec<_>>();

    let mut counter = 0;
    let mut out = "".to_string();
    for p1 in pairs.iter() {
        for p2 in pairs.iter() {
            let (_, pair_1st) = p1;
            let (_, pair_2nd) = p2;

            let (pair_1st_1st, pair_1st_2nd) = (pair_1st[0], pair_1st[1]);
            let (pair_2nd_1st, pair_2nd_2nd) = (pair_2nd[0], pair_2nd[1]);

            if pair_1st_1st != pair_1st_2nd {
                if pair_1st_2nd.0 != pair_2nd_1st.0
                    && pair_1st_1st != pair_2nd_1st
                    && pair_1st_2nd != pair_2nd_2nd
                    && pair_1st_1st != pair_2nd_2nd
                {
                    if pair_1st_1st.1 == pair_2nd_1st.1 && pair_1st_2nd.1 == pair_2nd_2nd.1 {
                        counter += 1;
                        let format = format!(
                            "{}, {}",
                            p1.1[0].1.to_string() + p2.1[0].1.to_string().as_str(),
                            p1.1[1].1.to_string() + p2.1[1].1.to_string().as_str()
                        );
                        out = format;
                    }
                }
            }
        }
    }

    (counter >= 1, out)
}

fn contains_palindrome(input: &str) -> (bool, String) {
    let chars = input.chars().collect::<Vec<_>>();

    let mut counter = 0;
    let mut out = "".to_string();
    for triplet in chars.windows(3) {
        if triplet[0] == triplet[2] {
            counter += 1;
            out = format!(
                "{}{}{}",
                triplet[0].clone(),
                triplet[1].clone(),
                triplet[2].clone()
            );
        };
    }

    (counter >= 1, out)
}

fn is_nice(input: &str) -> (bool, String, String) {
    let v1 = two_letter_pair(input);
    let v2 = contains_palindrome(input);

    (v1.0 && v2.0, v1.1, v2.1)
}

fn process(input: &str) -> usize {
    let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

    let mut nice_counter = 0;
    for line in lines {
        let is_nice = is_nice(line);
        if is_nice.0 {
            nice_counter += 1;
        }
        println!("{} => {:?}, {:?}", line, is_nice.1, is_nice.2);
    }

    nice_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    // IS NICE TESTS
    #[test]
    fn part2_test1() {
        let result = process("qjhvhtzxzqqjkmpb");
        assert_eq!(result, 1);
    }
    #[test]
    fn part2_test2() {
        let result = process("xxyxx");
        assert_eq!(result, 1);
    }
    #[test]
    fn part2_test3() {
        let result = process("uurcxstgmygtbstg");
        assert_eq!(result, 0);
    }
    #[test]
    fn part2_test4() {
        let result = process("ieodomkazucvgmuy");
        assert_eq!(result, 0);
    }
    // HAS TWO PAIRS OF DIFFERENT LETTERS TESTS
    #[test]
    fn two_letter_pair_test1() {
        let result = two_letter_pair("qjhvhtzxzqqjkmpb");
        assert_eq!(result.0, true);
    }
    #[test]
    fn two_letter_pair_test2() {
        let result = two_letter_pair("xxyxx");
        assert_eq!(result.0, true);
    }
    #[test]
    fn two_letter_pair_test3() {
        let result = two_letter_pair("uurcxstgmygtbstg");
        assert_eq!(result.0, true);
    }
    #[test]
    fn two_letter_pair_test4() {
        let result = two_letter_pair("ieodomkazucvgmuy");
        assert_eq!(result.0, false);
    }
    #[test]
    fn two_letter_pair_test5() {
        let result = two_letter_pair("aaa");
        assert_eq!(result.0, false);
    }
    // CONTAINS PALINDROM TESTS
    #[test]
    fn contains_palindrome_test1() {
        let result = contains_palindrome("qjhvhtzxzqqjkmpb");
        assert_eq!(result.0, true);
    }
    #[test]
    fn contains_palindrome_test2() {
        let result = contains_palindrome("xxyxx");
        assert_eq!(result.0, true);
    }
    #[test]
    fn contains_palindrome_test3() {
        let result = contains_palindrome("uurcxstgmygtbstg");
        assert_eq!(result.0, false);
    }
    #[test]
    fn contains_palindrome_test4() {
        let result = contains_palindrome("ieodomkazucvgmuy");
        assert_eq!(result.0, true);
    }
}
