use csr;

pub fn get_next_cmd() -> u32 {
    unsafe {
        csr::rabi::rabi_cntr0_write(0xffffffff);
        csr::rabi::rabi_cntr0_read()
    }
}
