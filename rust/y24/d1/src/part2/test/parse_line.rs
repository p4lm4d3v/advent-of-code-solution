#[cfg(test)]
mod t2 {
    use crate::parse_line;

    #[test]
    fn parse_line1() {
        let result = parse_line("3   4");
        assert_eq!(result, (3, 4));
    }
    #[test]
    fn parse_line2() {
        let result = parse_line("4   3");
        assert_eq!(result, (4, 3));
    }
    #[test]
    fn parse_line3() {
        let result = parse_line("2   5");
        assert_eq!(result, (2, 5));
    }
    #[test]
    fn parse_line4() {
        let result = parse_line("1   3");
        assert_eq!(result, (1, 3));
    }
    #[test]
    fn parse_line5() {
        let result = parse_line("3   9");
        assert_eq!(result, (3, 9));
    }
    #[test]
    fn parse_line6() {
        let result = parse_line("3   3");
        assert_eq!(result, (3, 3));
    }
}
