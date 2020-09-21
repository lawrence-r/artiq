use board_misoc::csr;

#[cfg(has_emulator)]
pub unsafe fn sdram_backdoor_copy(from: u32, to: u32, size: u32) { 
    csr::emulator::sdram_cp_from_write(from);
    csr::emulator::sdram_cp_to_write(to);
    csr::emulator::sdram_cp_size_write(size);
    csr::emulator::sdram_cp_write(1);
}

#[cfg(has_emulator)]
pub unsafe fn sdram_backdoor_erase(from: u32, to: u32) { 
    csr::emulator::sdram_clear_from_write(from);
    csr::emulator::sdram_clear_to_write(to);
    csr::emulator::sdram_clear_write(1);
}