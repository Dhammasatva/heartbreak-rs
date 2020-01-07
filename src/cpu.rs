use std::fmt;

#[derive(Default)]
pub struct CPU {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0xff,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            sp: 0xFFFF,
            pc: 0x0000,
        }
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print!("  a  b  c  d  e  h  l  sp  pc\n");
        writeln!(
            f,
            "{:>02x} {:>02x} {:>02x} {:>02x} {:>02x} {:>02x} {:>02x} {:>04x} {:>04x}",
            self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc
        )
    }
}
