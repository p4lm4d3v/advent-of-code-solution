#[cfg(test)]
mod t2 {
    use crate::contains_palindrome;

    #[test]
    fn contains_palindrome_test1() {
        let result = contains_palindrome("qjhvhtzxzqqjkmpb");
        assert_eq!(result.0, true);
    }
    #[test]
    fn contains_palindrome_test2() {
        let result = contains_palindrome("xxyxx");
        assert_eq!(result.0, true);
    }
    #[test]
    fn contains_palindrome_test3() {
        let result = contains_palindrome("uurcxstgmygtbstg");
        assert_eq!(result.0, false);
    }
    #[test]
    fn contains_palindrome_test4() {
        let result = contains_palindrome("ieodomkazucvgmuy");
        assert_eq!(result.0, true);
    }
}
