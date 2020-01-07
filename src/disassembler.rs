pub fn disassemble(buf: &[u8], pc: usize) -> usize {
    let mut op_offset: usize = 1;
    let op = buf[pc];

    match op {
        0x00 => { print_instruction("NOP", &buf[pc..pc+op_offset])},
        0x01 => { 
            op_offset = 3; 
            print_instruction("LXI B,", &buf[pc..pc+op_offset])
        },
        0x02 => { print_instruction("STAX B", &buf[pc.pc+op_offset]) },
        0x03 => { print_instruction("INX B", &buf[pc..pc+op_offset])},
        0x04 => { print_instruction("INR B", &buf[pc..pc+op_offset])},
        0x05 => { print_instruction("INX B", &buf[pc..pc+op_offset])},
        0x06 => { print_instruction("INX B", &buf[pc..pc+op_offset])},

        _ => {}
    }

    op_offset
}


fn print_instruction(instruction: &str, bytes: &[u8])  {
    match bytes.len() {
        1 => println!("{}", instruction),
        2 => {
            println!("{} {}", instruction, bytes[1]);
        },
        3 => {
            println!("{} {} {}", instruction, bytes[2], bytes[1]);
        },
        _ => {}
    }
}