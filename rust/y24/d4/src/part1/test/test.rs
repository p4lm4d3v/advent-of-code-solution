#[cfg(test)]
mod t1 {
    use crate::process;

    #[test]
    fn process1() {
        let result = process("..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....");
        assert_eq!(result, 4);
    }

    #[test]
    fn process2() {
        let result = process("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX");
        assert_eq!(result, 18);
    }

    #[test]
    fn process3() {
        let result = process("....XXMAS.\n.SAMXMS...\n...S..A...\n..A.A.MS.X\nXMASAMX.MM\nX.....XA.A\nS.S.S.S.SS\n.A.A.A.A.A\n..M.M.M.MM\n.X.X.XMASX");
        assert_eq!(result, 18);
    }
}
