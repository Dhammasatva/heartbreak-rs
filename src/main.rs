mod cpu;
mod disassembler;

use disassembler::disassemble;
fn main() {
    let mut test_buffer = Vec::new();
    test_buffer.push(0x00);
    let counter = 0_usize;
    disassemble(test_buffer.as_slice(), counter);
}
