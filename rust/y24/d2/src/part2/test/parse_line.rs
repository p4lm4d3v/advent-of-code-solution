#[cfg(test)]
mod t2 {
    use crate::parse_line;

    #[test]
    fn parse_line1() {
        let result = parse_line("7 6 4 2 1");
        assert_eq!(result, vec![7, 6, 4, 2, 1]);
    }
    #[test]
    fn parse_line2() {
        let result = parse_line("1 2 7 8 9");
        assert_eq!(result, vec![1, 2, 7, 8, 9]);
    }
    #[test]
    fn parse_line3() {
        let result = parse_line("9 7 6 2 1");
        assert_eq!(result, vec![9, 7, 6, 2, 1]);
    }
    #[test]
    fn parse_line4() {
        let result = parse_line("1 3 2 4 5");
        assert_eq!(result, vec![1, 3, 2, 4, 5]);
    }
    #[test]
    fn parse_line5() {
        let result = parse_line("8 6 4 4 1");
        assert_eq!(result, vec![8, 6, 4, 4, 1]);
    }
    #[test]
    fn parse_line6() {
        let result = parse_line("1 3 6 7 9");
        assert_eq!(result, vec![1, 3, 6, 7, 9]);
    }
}
