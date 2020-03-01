extern crate riscv_emu;

use std::env;
use std::fs;

use riscv_emu::exec::{step_rv32, ExecStatus};
use riscv_emu::isas::RV32;
use riscv_emu::{ops, Instruction, RawInstruction};
use riscv_emu::{AddressConverter, AddressTransformer, Bus, Memory};
use riscv_emu::{Hart, SingleThreadUserHart};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    match fs::read(filename) {
        Err(err) => {
            println!("failed to read the file: {:?}", err);
        }
        Ok(bytes) => exec_raw_program(bytes),
    }
}

fn exec_raw_program(mut img: Vec<u8>) {
    let start_pc = 0x80000000;
    let mem_buf = img.as_mut_slice();
    let mem = AddressTransformer::new(
        AddressConverter::new(Memory::new_ram(mem_buf)),
        |addr: u32| Ok(addr.wrapping_sub(start_pc)),
    );
    let mut hart: SingleThreadUserHart<RV32, _> = SingleThreadUserHart::new(mem);
    hart.write_pc(start_pc);

    let mut steps = 0;
    loop {
        if steps > 512 {
            panic!("test program is still running after 4096 steps, so aborting");
        }
        let pc = hart.read_pc();
        hart.with_memory(|mem| match mem.read_word(pc) {
            Ok(v) => {
                let raw_inst = RawInstruction::new(v);
                let inst: Instruction<ops::RV32, _> = Instruction::decode_raw(raw_inst, pc);
                println!("0x{:08x}: {:?}", pc, inst.op);
            }
            Err(_) => {
                println!("0x{:08x}: <failed to read instruction from memory>", pc);
            }
        });
        steps += 1;
        let status = step_rv32(&mut hart);
        match status {
            ExecStatus::EnvironmentCall(_) => break,
            _ => (),
        }
    }
}
