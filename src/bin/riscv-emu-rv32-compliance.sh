# This is a helper wrapper around running riscv-emu-rv32-compliance for a
# single upstream test case in a development environment, for quicker iteration.
#
# Run it from the root of this repository, like:
#     bash src/bin/riscv-emu-rv32-compliance.sh /path/to/riscv-compliance rv32i I-NOP-01
#
# You must've successfully run the full compliance test suite at least once
# with this emulator's target set in order for the test programs to be present
# and correct. For example, inside the riscv-compliance root directory:
#     make clean
#     make RISCV_PREFIX=riscv32-unknown-elf- RISCV_TARGET=rust-riscvemu TARGET_SIM=/path/to/rust-riscv-emu/target/debug/riscv-emu-rv32-compliance

set -euo pipefail

COMPLIANCE_BASE_DIR="$1"
TEST_SUITE_NAME="$2"
TEST_CASE_NAME="$3"

cargo run "$COMPLIANCE_BASE_DIR/work/$TEST_SUITE_NAME/$TEST_CASE_NAME.elf.bin" "$COMPLIANCE_BASE_DIR/work/$TEST_SUITE_NAME/$TEST_CASE_NAME.elf.syms" "$COMPLIANCE_BASE_DIR/work/$TEST_SUITE_NAME/$TEST_CASE_NAME.signature.output"
