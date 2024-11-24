#[cfg(test)]
mod t1 {
    use crate::process;

    #[test]
    fn test1() {
        let result = process("ugknbfddgicrmopn");
        assert_eq!(result, 1);
    }
    #[test]
    fn test2() {
        let result = process("jchzalrnumimnmhp");
        assert_eq!(result, 0);
    }
}
