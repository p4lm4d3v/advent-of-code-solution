#[cfg(test)]
mod t2 {
    use crate::process;

    #[test]
    fn test1() {
        let result = process("qjhvhtzxzqqjkmpb");
        assert_eq!(result, 1);
    }
    #[test]
    fn test2() {
        let result = process("xxyxx");
        assert_eq!(result, 1);
    }
    #[test]
    fn test3() {
        let result = process("uurcxstgmygtbstg");
        assert_eq!(result, 0);
    }
    #[test]
    fn test4() {
        let result = process("ieodomkazucvgmuy");
        assert_eq!(result, 0);
    }
}
