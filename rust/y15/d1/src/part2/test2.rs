#[cfg(test)]
mod t2 {
    use crate::process;

    #[test]
    fn test1() {
        assert_eq!(1, process(")"));
    }
    #[test]
    fn test2() {
        assert_eq!(5, process("()())"));
    }
    #[test]
    fn test3() {
        assert_eq!(51, process("((((((((((((((((((((((((())))))))))))))))))))))))))"));
    }
    #[test]
    fn test4() {
        assert_eq!(67, process("()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()())"));
    }
}
