#[cfg(test)]
mod t1 {
    use crate::process;

    #[test]
    fn part1_test1() {
        let result = process(">");
        assert_eq!(result, 2);
    }
    #[test]
    fn part1_test2() {
        let result = process("^>v<");
        assert_eq!(result, 4);
    }
    #[test]
    fn part1_test3() {
        let result = process("^v^v^v^v^v");
        assert_eq!(result, 2);
    }
}
