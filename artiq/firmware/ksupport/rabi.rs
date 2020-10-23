#[cfg(has_rabi)]
pub mod imp {
    use board_misoc::csr;

    pub extern fn read() -> u32 {
        unsafe {
            csr::rabi::rabi_cntr0_read() as u32
        }
    }
    
    pub extern fn write(data: u32) {
        unsafe {
            //csr::rtio::target_write(1 << 8);
            csr::rabi::rabi_cntr0_write(data as u32);
        }
    }
}

#[cfg(not(has_rabi))]
pub mod imp {
    pub extern fn write(data: u32) {
        unimplemented!("not(has_rabi)")
    }
    pub extern fn read() -> u32 {
        unimplemented!("not(has_rabi)")
    }
}
pub use self::imp::*;
