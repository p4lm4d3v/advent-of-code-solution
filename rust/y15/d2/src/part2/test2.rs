#[cfg(test)]
mod t2 {
  use crate::process;

    #[test]
    fn test1() {
        let result = process("2x3x4");
        assert_eq!(result, 34);
    }
    #[test]
    fn test2() {
        let result = process("1x1x10");
        assert_eq!(result, 14);
    }
}
