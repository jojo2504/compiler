mod tests {
    use compiler::code::{Opcode, make};

    #[test]
    fn test_make() {
        struct TestCase {
            op: Opcode,
            operands: Vec<i32>,
            expected: Vec<u8>,
        }

        let tests = vec![TestCase {
            op: Opcode::Constant,
            operands: vec![65534],
            expected: vec![Opcode::Constant as u8, 255, 254],
        }];

        for tt in tests {
            let instruction = make(tt.op, tt.operands.as_slice());

            if instruction.len() != tt.expected.len() {
                panic!(
                    "instruction has wrong length. want={}, got={}",
                    tt.expected.len(),
                    instruction.len()
                )
            }

            for (i, b) in tt.expected.iter().enumerate() {
                if instruction[i] != tt.expected[i] {
                    panic!(
                        "wrong byte at pos {}. want={}, got={}",
                        i, b, instruction[i]
                    )
                }
            }
        }
    }
}
