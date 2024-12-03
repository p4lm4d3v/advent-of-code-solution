#[cfg(test)]
mod t1 {
    use crate::{steps_rise_fall, Direction};

    #[test]
    fn steps_up_down1() {
        let result = steps_rise_fall(&vec![-1, -2, -2, -1]);
        assert_eq!(result, Direction::Falling);
    }
    #[test]
    fn steps_up_down2() {
        let result = steps_rise_fall(&vec![1, 5, 1, 1]);
        assert_eq!(result, Direction::Rising);
    }
    #[test]
    fn steps_up_down3() {
        let result = steps_rise_fall(&vec![-2, -1, -4, -1]);
        assert_eq!(result, Direction::Falling);
    }
    #[test]
    fn steps_up_down4() {
        let result = steps_rise_fall(&vec![2, -1, 2, 1]);
        assert_eq!(result, Direction::Unknown);
    }
    #[test]
    fn steps_up_down5() {
        let result = steps_rise_fall(&vec![-2, -2, 0, -3]);
        assert_eq!(result, Direction::Unknown);
    }
    #[test]
    fn steps_up_down6() {
        let result = steps_rise_fall(&vec![2, 3, 1, 2]);
        assert_eq!(result, Direction::Rising);
    }
}
