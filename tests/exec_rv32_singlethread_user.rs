use core::{concat, stringify};
use std::io;
use std::io::BufRead;

use riscv_emu::exec::{step_rv32, ExecStatus};
use riscv_emu::isas::RV32;
use riscv_emu::Int;
use riscv_emu::IntRegister;
use riscv_emu::{ops, Instruction, RawInstruction};
use riscv_emu::{AddressConverter, AddressTransformer, Bus, Memory, MemoryError};
use riscv_emu::{Hart, SingleThreadUserHart};

macro_rules! rv32case {
    ($filename:ident, $result_base:expr) => {
        #[test]
        fn $filename() {
            test_case(
                include_bytes!(concat!("rv32cases/", stringify!($filename), ".bin")),
                include_bytes!(concat!("rv32cases/", stringify!($filename), ".want")),
                $result_base,
            );
        }
    };
}

// Tests for the "I" integer base ISA
rv32case!(I_ADDI_01, 0x80002000);
rv32case!(I_ADD_01, 0x80002000);
rv32case!(I_ANDI_01, 0x80002000);
rv32case!(I_AND_01, 0x80002000);
rv32case!(I_AUIPC_01, 0x80002000);
rv32case!(I_BEQ_01, 0x80002000);
rv32case!(I_BGEU_01, 0x80002000);
rv32case!(I_BGE_01, 0x80002000);
rv32case!(I_BLTU_01, 0x80002000);
rv32case!(I_BLT_01, 0x80002000);
rv32case!(I_BNE_01, 0x80002000);
rv32case!(I_DELAY_SLOTS_01, 0x80002000);
//rv32case!(I_EBREAK_01, 0x80002000); // (requires exception handling)
//rv32case!(I_ECALL_01, 0x80002000); // (requires exception handling)
rv32case!(I_ENDIANESS_01, 0x80002010);
rv32case!(I_IO, 0x80002030);
rv32case!(I_JALR_01, 0x80002000);
rv32case!(I_JAL_01, 0x80002000);
rv32case!(I_LBU_01, 0x80002030);
rv32case!(I_LB_01, 0x80002030);
rv32case!(I_LHU_01, 0x80002030);
rv32case!(I_LH_01, 0x80002030);
rv32case!(I_LUI_01, 0x80002000);
rv32case!(I_LW_01, 0x80002030);
//rv32case!(I_MISALIGN_JMP_01, 0x80002000); // (requires exception handling)
//rv32case!(I_MISALIGN_LDST_01, 0x80002010); // (requires exception handling)
rv32case!(I_NOP_01, 0x80002000);
rv32case!(I_ORI_01, 0x80002000);
rv32case!(I_OR_01, 0x80002000);
rv32case!(I_RF_size_01, 0x80002000);
rv32case!(I_RF_width_01, 0x80002000);
rv32case!(I_RF_x0_01, 0x80002010);
rv32case!(I_SB_01, 0x80002000);
rv32case!(I_SH_01, 0x80002000);
rv32case!(I_SLLI_01, 0x80002000);
rv32case!(I_SLL_01, 0x80002000);
rv32case!(I_SLTIU_01, 0x80002000);
rv32case!(I_SLTI_01, 0x80002000);
rv32case!(I_SLTU_01, 0x80002000);
rv32case!(I_SLT_01, 0x80002000);
rv32case!(I_SRAI_01, 0x80002000);
rv32case!(I_SRA_01, 0x80002000);
rv32case!(I_SRLI_01, 0x80002000);
rv32case!(I_SRL_01, 0x80002000);
rv32case!(I_SUB_01, 0x80002000);
rv32case!(I_SW_01, 0x80002000);
rv32case!(I_XORI_01, 0x80002000);
rv32case!(I_XOR_01, 0x80002000);

// Tests for the "M" (Multiply) extension
rv32case!(DIV, 0x80002000);
rv32case!(DIVU, 0x80002000);
rv32case!(MUL, 0x80002000);
rv32case!(MULH, 0x80002000);
rv32case!(MULHSU, 0x80002000);
rv32case!(MULHU, 0x80002000);
rv32case!(REM, 0x80002000);
rv32case!(REMU, 0x80002000);

// Tests for the "C" (Compressed) extension
rv32case!(C_ADD, 0x80002000);
rv32case!(C_ADDI, 0x80002000);
rv32case!(C_ADDI16SP, 0x80002000);
rv32case!(C_ADDI4SPN, 0x80002000);
rv32case!(C_AND, 0x80002000);
rv32case!(C_ANDI, 0x80002000);
rv32case!(C_BEQZ, 0x80002000);
rv32case!(C_BNEZ, 0x80002000);
rv32case!(C_J, 0x80002000);
rv32case!(C_JAL, 0x80002000);
rv32case!(C_JALR, 0x80002000);
rv32case!(C_JR, 0x80002000);
rv32case!(C_LI, 0x80002000);
rv32case!(C_LUI, 0x80002000);
//rv32case!(C_LW, 0x80002080);
//rv32case!(C_LWSP, 0x80002100);
//rv32case!(C_MV, 0x80002000);
//rv32case!(C_OR, 0x80002000);
rv32case!(C_SLLI, 0x80002000);
//rv32case!(C_SRAI, 0x80002000);
//rv32case!(C_SRLI, 0x80002000);
rv32case!(C_SUB, 0x80002000);
//rv32case!(C_SW, 0x80002000);
//rv32case!(C_SWSP, 0x80002000);
rv32case!(C_XOR, 0x80002000);

fn test_case(img: &[u8], want_raw: &[u8], sig_start: u32) {
    let start_pc = 0x80000000;
    let mut mem_vec = img.to_owned();
    let mem_buf = mem_vec.as_mut_slice();
    let mem = MemLogger::new(AddressTransformer::new(
        AddressConverter::new(Memory::new_ram(mem_buf)),
        |addr: u32| Ok(addr.wrapping_sub(start_pc)),
    ));
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
    let mut mismatches = 0;
    let mut total = 0;
    let mut header = false;
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
                header = false;
                continue;
            }

            if !header {
                println!("== Test {} results", test_idx);
                header = true;
            }

            if let Ok(want) = u32::from_str_radix(&line, 16) {
                hart.with_memory(|mem| {
                    let addr = sig_start + offset;
                    match mem.read_word(sig_start + offset) {
                        Ok(got) => {
                            if got == want {
                                println!(
                                    "  OK {:3} 0x{:08x}                   @0x{:08x} (base+{:02}, test+{:02})",
                                    result_idx, got, addr, offset, local_offset,
                                );
                            } else {
                                mismatches += 1;
                                println!(
                                    "FAIL {:3} 0x{:08x} (want 0x{:08x}) @0x{:08x} (base+{:02}, test+{:02})",
                                    result_idx, got, want, addr, offset, local_offset,
                                );
                            }
                        }
                        Err(e) => {
                            println!("FAIL {:3} 0x???????? ({:?})", result_idx, e);
                            mismatches += 1;
                        }
                    }
                });
                offset += 4;
                local_offset += 4;
                result_idx += 1;
                total += 1;
            } else {
                panic!("'want' file line {} has invalid syntax", line_num);
            }

            line_num += 1;
        }
    }
    if mismatches > 0 {
        panic!("{} of {} test results are incorrect", mismatches, total);
    }
}

struct MemLogger<Wrapped: Bus<u32>> {
    wrapped: Wrapped,
}

impl<Wrapped: Bus<u32>> MemLogger<Wrapped> {
    fn new(wrapped: Wrapped) -> Self {
        Self { wrapped: wrapped }
    }
}

impl<Wrapped: Bus<u32>> Bus<u32> for MemLogger<Wrapped> {
    fn read_byte(&mut self, addr: u32) -> Result<u8, MemoryError> {
        self.wrapped.read_byte(addr)
    }
    fn read_halfword(&mut self, addr: u32) -> Result<u16, MemoryError> {
        self.wrapped.read_halfword(addr)
    }
    fn read_word(&mut self, addr: u32) -> Result<u32, MemoryError> {
        self.wrapped.read_word(addr)
    }
    fn read_longword(&mut self, addr: u32) -> Result<u64, MemoryError> {
        self.wrapped.read_longword(addr)
    }
    fn read_quadword(&mut self, addr: u32) -> Result<u128, MemoryError> {
        self.wrapped.read_quadword(addr)
    }

    fn write_byte(&mut self, addr: u32, v: u8) -> Result<(), MemoryError> {
        println!("- write_byte(0x{:08x}, 0x{:02x})", addr, v);
        self.wrapped.write_byte(addr, v)
    }
    fn write_halfword(&mut self, addr: u32, v: u16) -> Result<(), MemoryError> {
        println!("- write_halfword(0x{:08x}, 0x{:04x})", addr, v);
        self.wrapped.write_halfword(addr, v)
    }
    fn write_word(&mut self, addr: u32, v: u32) -> Result<(), MemoryError> {
        println!("- write_word(0x{:08x}, 0x{:08x})", addr, v);
        self.wrapped.write_word(addr, v)
    }
    fn write_longword(&mut self, addr: u32, v: u64) -> Result<(), MemoryError> {
        println!("- write_longword(0x{:08x}, 0x{:016x})", addr, v);
        self.wrapped.write_longword(addr, v)
    }
    fn write_quadword(&mut self, addr: u32, v: u128) -> Result<(), MemoryError> {
        println!("- write_quadword(0x{:08x}, 0x{:032x})", addr, v);
        self.wrapped.write_quadword(addr, v)
    }
}
