#[cfg(test)]
mod t1 {
    use crate::process;

    #[test]
    fn test1() {
        let result = process("abcdef");
        assert_eq!(result, 609043);
    }
    #[test]
    fn test2() {
        let result = process("pqrstuv");
        assert_eq!(result, 1048970);
    }
    #[test]
    fn test3() {
        let result = process("input");
        assert_eq!(result, 1256406);
    }
    #[test]
    fn test4() {
        let result = process("output");
        assert_eq!(result, 1049475);
    }
}
