extern crate riscv_emu;

use std::env;
use std::fs;
use std::io;
use std::io::{BufRead, Write};

use riscv_emu::exec::{step_rv32, ExecStatus};
use riscv_emu::isas::RV32;
use riscv_emu::Int;
use riscv_emu::IntRegister;
use riscv_emu::{ops, Instruction, RawInstruction};
use riscv_emu::{AddressConverter, AddressTransformer, Bus, Memory};
use riscv_emu::{Hart, SingleThreadUserHart};

fn main() {
    let args: Vec<String> = env::args().collect();
    let bin_filename = &args[1];
    let syms_filename = &args[2];
    let sig_filename = &args[3];

    // Because our input is a raw binary image, it doesn't contain any symbol
    // table information. We need to know the range of bytes in RAM where the
    // output signature will be found though, so we "parse" another file which
    // is assumed to be output from "objdump -t" to discover the addresses
    // of the begin_signature and end_signature symbols, which is the slice
    // of memory we need to capture for comparison with the reference data
    // in the test suite.
    let syms_file = fs::File::open(syms_filename).unwrap();
    let mut sig_begin: u32 = 0;
    let mut sig_end: u32 = 0;
    for line_result in io::BufReader::new(syms_file).lines() {
        if let Ok(line) = line_result {
            if line.len() < 20 {
                // Skip lines that can't possibly be long enough for what
                // we're looking for, because they don't have enough characters
                // for both an address and a symbol name.
                continue;
            }
            if line.contains("begin_signature") {
                if let Ok(v) = u32::from_str_radix(&line[0..8], 16) {
                    sig_begin = v
                }
            }
            if line.contains("end_signature") {
                if let Ok(v) = u32::from_str_radix(&line[0..8], 16) {
                    sig_end = v
                }
            }
        }
    }
    let sig_range = sig_begin..sig_end;

    let mut sig_file = fs::File::create(sig_filename).unwrap();

    match fs::read(bin_filename) {
        Err(err) => {
            println!("failed to read the file: {:?}", err);
        }
        Ok(bytes) => exec_raw_program(bytes, sig_range, sig_file),
    }
}

fn exec_raw_program(mut img: Vec<u8>, sig_range: std::ops::Range<u32>, mut sig_file: fs::File) {
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
        if steps >= 512 {
            panic!(
                "test program is still running after {} steps, so aborting",
                steps
            );
        }
        let pc = hart.read_pc();
        hart.with_memory(|mem| match mem.read_word(pc) {
            Ok(v) => {
                let raw_inst = RawInstruction::new(v);
                let inst: Instruction<ops::RV32, _> = Instruction::decode_raw(raw_inst, pc);
                println!("0x{:08x}: {:?}", pc, inst.op);
                if let ops::RV32::Invalid = inst.op {
                    panic!("hit invalid instruction, so aborting");
                }
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

    // By convention the tests leave their own sense of the test result in
    // the x3 (gp) register. This is not necessarily trustworthy, but we'll
    // capture it anyway as a pre-check before we check the output data.
    let result = hart
        .read_int_register(IntRegister::numbered(3))
        .to_unsigned();
    println!("after test, x3 register contains 0x{:08x}", result);

    hart.with_memory(|mem| {
        for addr in sig_range {
            if (addr & 0b11) != 0 {
                continue; // only whole words
            }
            match mem.read_word(addr) {
                Ok(v) => writeln!(sig_file, "{:08x}", v),
                Err(e) => writeln!(sig_file, "XXXXXXXX"),
            };
        }
    })
}
