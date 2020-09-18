#[cfg(has_rabi)]
pub mod imp {
    use core::ptr::{read_volatile, write_volatile};
    use cslice::CSlice;
    use board_misoc::csr;
    use ::send;
    use ::recv;
    use kernel_proto::*;

    #[inline(always)]
    pub extern fn read() -> u32 {
        unsafe {
            csr::rabi::rabi_cntr0_read() as u32
        }
    }
    #[inline(always)]
    pub extern fn write(data: u32) {
        unsafe {
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
