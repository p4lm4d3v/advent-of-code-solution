#[cfg(test)]
mod test1 {
    use crate::process;

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
        assert_eq!(-1, process("))("));
    }
    #[test]
    fn test8() {
        assert_eq!(-3, process(")))"));
    }
    #[test]
    fn test9() {
        assert_eq!(-3, process(")())())"));
    }
}
