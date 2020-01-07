

#[derive(Default)]
pub struct mnemonic {
    opcode: u8,
    instruction_name: String,
    size: u8
}

impl mnemonic { 
    pub fn new(opcode: u8, instruction_name: u8, size: u8) -> Self { 
        mnemonic {
            opcode: opcode,
            instruction_name: instruction_name,
            size: size
        }
    }
}

pub fn disassemble(buf: &[u8], pc: &mut usize) -> mnemonic {

}