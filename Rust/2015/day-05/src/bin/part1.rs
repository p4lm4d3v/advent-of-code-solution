fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
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
        println!("{}", line);
        if is_nice(line) {
            nice_counter += 1;
        }
    }

    nice_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    // IS NICE TESTS
    #[test]
    fn part1_test1() {
        let result = process("ugknbfddgicrmopn");
        assert_eq!(result, 1);
    }
    #[test]
    fn part1_test2() {
        let result = process("jchzalrnumimnmhp");
        assert_eq!(result, 0);
    }
    // HAS AT LEAST 3 VOWELS TESTS
    #[test]
    fn has_at_least_3_vowels_test1() {
        let result1 = has_at_least_3_vowels("aio");
        assert_eq!(result1, true);
    }
    #[test]
    fn has_at_least_3_vowels_test2() {
        let result2 = has_at_least_3_vowels("xazegov");
        assert_eq!(result2, true);
    }
    #[test]
    fn has_at_least_3_vowels_test3() {
        let result3 = has_at_least_3_vowels("pqr");
        assert_eq!(result3, false);
    }
    #[test]
    fn has_at_least_3_vowels_test4() {
        let result4 = has_at_least_3_vowels("aaa");
        assert_eq!(result4, true);
    }
    // HAS LEETER TWICE IN A ROW TEST
    #[test]
    fn letter_twice_in_a_row_test1() {
        let result1 = letter_twice_in_a_row("aaa");
        assert_eq!(result1, true);
    }
    #[test]
    fn letter_twice_in_a_row_test2() {
        let result2 = letter_twice_in_a_row("aabbccdd");
        assert_eq!(result2, true);
    }

    #[test]
    fn letter_twice_in_a_row_test3() {
        let result3 = letter_twice_in_a_row("abc");

        assert_eq!(result3, false);
    }
    // DOES NOT CONTAIN TESTS
    #[test]
    fn does_not_contain_test1() {
        let result1 = does_not_contain("ab");
        assert_eq!(result1, false);
    }
    #[test]
    fn does_not_contain_test2() {
        let result2 = does_not_contain("abcdpqxy");
        assert_eq!(result2, false);
    }
    #[test]
    fn does_not_contain_test3() {
        let result3 = does_not_contain("godhsr");
        assert_eq!(result3, true);
    }
}
