use core::{concat, stringify};
use std::io;
use std::io::BufRead;

use riscv_emu::exec::{step_rv32, ExecStatus};
use riscv_emu::isas::RV32;
use riscv_emu::Int;
use riscv_emu::IntRegister;
use riscv_emu::{ops, Instruction, RawInstruction};
use riscv_emu::{AddressConverter, AddressTransformer, Bus, Memory};
use riscv_emu::{Hart, SingleThreadUserHart};

macro_rules! rv32case {
    ($filename:ident) => {
        #[test]
        fn $filename() {
            test_case(
                include_bytes!(concat!("rv32cases/", stringify!($filename), ".bin")),
                include_bytes!(concat!("rv32cases/", stringify!($filename), ".want")),
                0x80002000,
            );
        }
    };
}

rv32case!(MUL);

fn test_case(img: &[u8], want_raw: &[u8], sig_start: u32) {
    let start_pc = 0x80000000;
    let mut mem_vec = img.to_owned();
    let mem_buf = mem_vec.as_mut_slice();
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

    // If we ran to completion then we'll read in the "want" data and compare
    // that with the real test result data in the hart's memory.
    let want_rd = io::Cursor::new(want_raw);
    let mut offset = 0;
    let mut local_offset = 0;
    let mut test_idx = 0;
    let mut result_idx = 0;
    let mut line_num = 1;
    for line_result in io::BufReader::new(want_rd).lines() {
        if let Ok(line) = line_result {
            if line.len() == 0 {
                continue;
            }
            if line.starts_with("-") {
                // Beginning a new test
                test_idx += 1;
                result_idx = 0;
                local_offset = 0;
                continue;
            }

            if let Ok(want) = u32::from_str_radix(&line, 16) {
                hart.with_memory(|mem| {
                    let addr = sig_start + offset;
                    match mem.read_word(sig_start + offset) {
                        /*Ok(got) => assert_eq!(
                            got, want,
                            "test {} result {} (at 0x{:08x}, abs offset 0x{:x}, local offset 0x{:x})",
                            test_idx, result_idx, addr, offset, local_offset
                        ),*/
                        Ok(got) => assert!(
                            got == want,
                            "wrong value for test {} result {}\ngot:  0x{:08x}\nwant: 0x{:08x}\n(at 0x{:08x}, abs offset 0x{:x}, local offset 0x{:x})\n",
                            test_idx, result_idx, got, want, addr, offset, local_offset
                        ),
                        Err(e) => panic!(
                            "failed to read test {} result {} (at 0x{:08x}: abs offset 0x{:x}, local offset 0x{:x}): {:?}",
                            test_idx, result_idx, addr, offset, local_offset, e
                        ),
                    }
                });
                offset += 4;
                local_offset += 4;
                result_idx += 1;
            } else {
                panic!("'want' file line {} has invalid syntax", line_num);
            }

            line_num += 1;
        }
    }
}
