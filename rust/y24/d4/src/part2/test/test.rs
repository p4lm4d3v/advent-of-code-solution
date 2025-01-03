#[cfg(test)]
mod t2 {
    use crate::process;

    #[test]
    fn process1() {
        let result = process("M.S\n.A.\nM.S");
        assert_eq!(result, 1);
    }

    #[test]
    fn process2() {
        let result = process(".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n..........");
        assert_eq!(result, 9);
    }
}
