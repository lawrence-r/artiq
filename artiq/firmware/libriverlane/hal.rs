
    use csr;
    //use board_misoc::csr;
    use core::ptr::{read_volatile, write_volatile};


    pub const RTIO_O_STATUS_WAIT:                      u8 = 1;
    pub const RTIO_O_STATUS_UNDERFLOW:                 u8 = 2;
    pub const RTIO_O_STATUS_DESTINATION_UNREACHABLE:   u8 = 4;
    pub const RTIO_I_STATUS_WAIT_EVENT:                u8 = 1;
    pub const RTIO_I_STATUS_OVERFLOW:                  u8 = 2;
    pub const RTIO_I_STATUS_WAIT_STATUS:               u8 = 4;
    pub const RTIO_I_STATUS_DESTINATION_UNREACHABLE:   u8 = 8;

    // writing the LSB of o_data (offset=0) triggers the RTIO write
    #[inline(always)]
    pub unsafe fn rtio_o_data_write(offset: usize, data: u32) {
        write_volatile(
            csr::rtio::O_DATA_ADDR.offset((csr::rtio::O_DATA_SIZE - 1 - offset) as isize),
            data);
    }

    #[inline(always)]
    pub unsafe fn rtio_i_data_read(offset: usize) -> u32 {
        read_volatile(
            csr::rtio::I_DATA_ADDR.offset((csr::rtio::I_DATA_SIZE - 1 - offset) as isize))
    }

    pub extern fn get_counter() -> i64 {
        unsafe {
            csr::rtio::counter_update_write(1);
            csr::rtio::counter_read() as i64
        }
    }
    
    #[inline(never)]
    pub fn process_exceptional_status(status: u8) -> &'static str {
        unsafe {

        if status & RTIO_O_STATUS_WAIT != 0 {
            while csr::rtio::o_status_read() & RTIO_O_STATUS_WAIT != 0 {}
        }
        if status & RTIO_O_STATUS_UNDERFLOW != 0 {
            return "RTIO underflow"; 
        }
        if status & RTIO_O_STATUS_DESTINATION_UNREACHABLE != 0 {
            return "RTIO destination unreachable";
        }
        return "RTIO_OK";
        }   
    }



    pub fn get_next_cmd() -> Option<u32> {
        unsafe {
            csr::hal::hal_cntr0_write(0xffffffff);
            let cmd = csr::hal::hal_cntr0_read();
            if cmd == 0 {
                None
            } else {
                Some(cmd)
            }
            
        }   
    }

    pub fn send_cmd_to_rtio(cmd: u32) -> u8{
        unsafe {
            csr::rtio::target_write(0 as u32);
            // writing target clears o_data
            rtio_o_data_write(0, cmd as _);
            csr::rtio::o_status_read()
        }
    }

