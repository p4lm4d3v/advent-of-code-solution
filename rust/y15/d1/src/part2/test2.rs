#[cfg(test)]
mod test2 {
    use crate::process;

    #[test]
    fn test1() {
        assert_eq!(1, process(")"));
    }
    #[test]
    fn test2() {
        assert_eq!(5, process("()())"));
    }
}
