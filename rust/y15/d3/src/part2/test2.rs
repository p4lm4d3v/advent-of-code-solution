#[cfg(test)]
mod tests2 {
    use crate::process;

    #[test]
    fn part2_test1() {
        let result = process("^v");
        assert_eq!(result, 3);
    }
    #[test]
    fn part2_test2() {
        let result = process("^>v<");
        assert_eq!(result, 3);
    }
    #[test]
    fn part2_test3() {
        let result = process("^v^v^v^v^v");
        assert_eq!(result, 11);
    }
}
