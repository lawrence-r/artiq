#[cfg(has_hal)]
pub mod imp {
    use board_misoc::csr;

    pub extern fn read() -> u32 {
        unsafe {
            csr::hal::hal_cntr0_read() as u32
        }
    }
    
    pub extern fn write(data: u32) {
        unsafe {
            csr::hal::hal_cntr0_write(data as u32);
        }
    }
}

#[cfg(not(has_hal))]
pub mod imp {
    pub extern fn write(data: u32) {
        unimplemented!("not(has_hal)")
    }
    pub extern fn read() -> u32 {
        unimplemented!("not(has_hal)")
    }
}
pub use self::imp::*;
