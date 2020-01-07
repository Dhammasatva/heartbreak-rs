mod cpu;
use cpu::CPU;

fn main() {
    let test_cpu = CPU::new();
    println!("{:?}", test_cpu);
}
