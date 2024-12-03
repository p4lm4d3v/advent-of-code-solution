#[cfg(test)]
mod t1 {
    use crate::steps_in_interval;

    #[test]
    fn steps_in_interval1() {
        let result = steps_in_interval(&vec![-1, -2, -2, -1]);
        assert!(result);
    }
    #[test]
    fn steps_in_interval2() {
        let result = steps_in_interval(&vec![1, 5, 1, 1]);
        assert!(!result);
    }
    #[test]
    fn steps_in_interval3() {
        let result = steps_in_interval(&vec![-2, -1, -4, -1]);
        assert!(!result);
    }
    #[test]
    fn steps_in_interval4() {
        let result = steps_in_interval(&vec![2, -1, 2, 1]);
        assert!(result);
    }
    #[test]
    fn steps_in_interval5() {
        let result = steps_in_interval(&vec![-2, -2, 0, -3]);
        assert!(!result);
    }
    #[test]
    fn steps_in_interval6() {
        let result = steps_in_interval(&vec![2, 3, 1, 2]);
        assert!(result);
    }
}
