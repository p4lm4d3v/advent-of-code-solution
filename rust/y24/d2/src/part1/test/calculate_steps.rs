#[cfg(test)]
mod t1 {
    use crate::calculate_steps;

    #[test]
    fn calculate_steps1() {
        let result = calculate_steps(vec![7, 6, 4, 2, 1]);
        assert_eq!(result, vec![-1, -2, -2, -1])
    }
    #[test]
    fn calculate_steps2() {
        let result = calculate_steps(vec![1, 2, 7, 8, 9]);
        assert_eq!(result, vec![1, 5, 1, 1])
    }
    #[test]
    fn calculate_steps3() {
        let result = calculate_steps(vec![9, 7, 6, 2, 1]);
        assert_eq!(result, vec![-2, -1, -4, -1])
    }
    #[test]
    fn calculate_steps4() {
        let result = calculate_steps(vec![1, 3, 2, 4, 5]);
        assert_eq!(result, vec![2, -1, 2, 1])
    }
    #[test]
    fn calculate_steps5() {
        let result = calculate_steps(vec![8, 6, 4, 4, 1]);
        assert_eq!(result, vec![-2, -2, 0, -3])
    }
    #[test]
    fn calculate_steps6() {
        let result = calculate_steps(vec![1, 3, 6, 7, 9]);
        assert_eq!(result, vec![2, 3, 1, 2])
    }
}
