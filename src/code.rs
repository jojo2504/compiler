use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

pub type Instructions = Vec<u8>;

macro_rules! definitions {
    () => {
        DEFINITIONS.lock().unwrap()
    };
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
#[repr(u8)]
pub enum Opcode {
    Constant,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Constant,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
pub struct Definition {
    name: &'static str,
    operand_widths: Vec<i32>,
}

pub static DEFINITIONS: LazyLock<Mutex<HashMap<Opcode, Definition>>> = LazyLock::new(|| {
    let _definitions: HashMap<Opcode, Definition> = HashMap::from([(
        Opcode::Constant,
        Definition {
            name: "OpConstant",
            operand_widths: vec![2],
        },
    )]);
    Mutex::new(_definitions)
});

pub fn lookup(op: u8) -> Result<Definition, String> {
    definitions!()
        .get(&Opcode::from(op))
        .cloned()
        .ok_or(format!("opcode {} undefined", op))
}

pub fn make(op: Opcode, operands: &[i32]) -> Vec<u8> {
    let definition = definitions!()
        .get(&op)
        .cloned()
        .ok_or(format!("opcode {:?} undefined", op));

    let definition = if let Ok(definition) = definition {
        definition
    } else {
        return vec![];
    };

    let mut instruction_len = 1;
    for w in &definition.operand_widths {
        instruction_len += w;
    }

    let mut instruction: Instructions = vec![0u8; instruction_len as usize];
    instruction[0] = op as u8;

    let mut offset = 1;
    for (i, &o) in operands.iter().enumerate() {
        let width = definition.operand_widths[i];
        match width {
            2 => {
                let bytes = (o as u16).to_be_bytes();
                instruction[offset..offset + 2].copy_from_slice(&bytes);
            }
            _ => panic!(),
        }
        offset += width as usize;
    }

    instruction
}
