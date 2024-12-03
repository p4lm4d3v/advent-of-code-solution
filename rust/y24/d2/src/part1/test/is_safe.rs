#[cfg(test)]
mod t1 {
    use crate::is_safe;

    #[test]
    fn line_safe1() {
        let result = is_safe("7 6 4 2 1");
        assert!(result);
    }
    #[test]
    fn line_safe2() {
        let result = is_safe("1 2 7 8 9");
        assert!(!result);
    }
    #[test]
    fn line_safe3() {
        let result = is_safe("9 7 6 2 1");
        assert!(!result);
    }
    #[test]
    fn line_safe4() {
        let result = is_safe("1 3 2 4 5");
        assert!(!result);
    }
    #[test]
    fn line_safe5() {
        let result = is_safe("8 6 4 4 1");
        assert!(!result);
    }
    #[test]
    fn line_safe6() {
        let result = is_safe("1 3 6 7 9");
        assert!(result);
    }
}
