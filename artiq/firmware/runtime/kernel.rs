use core::ptr;
use board_misoc::csr;
use mailbox;
use rpc_queue;

use kernel_proto::{KERNELCPU_EXEC_ADDRESS, KERNELCPU_LAST_ADDRESS, KSUPPORT_HEADER_SIZE};


#[cfg(has_kernel_cpu)]
pub unsafe fn start() {
    if csr::kernel_cpu::reset_read() == 0 {
        panic!("attempted to start kernel CPU when it is already running")
    }

    stop();

    extern {
        static _binary____ksupport_ksupport_elf_start: u8;
        static _binary____ksupport_ksupport_elf_end: u8;
    }
    let ksupport_start = &_binary____ksupport_ksupport_elf_start as *const _;
    let ksupport_end   = &_binary____ksupport_ksupport_elf_end as *const _;
    info!("kernel:start - copy_nonoverlapping {} bytes at {:x}", 
        ksupport_end as usize - ksupport_start as usize, KERNELCPU_EXEC_ADDRESS - KSUPPORT_HEADER_SIZE);
    
//    #[cfg(not(has_emulator))]
//    ptr::copy_nonoverlapping(ksupport_start,
//                             (KERNELCPU_EXEC_ADDRESS - KSUPPORT_HEADER_SIZE) as *mut u8,
//                             ksupport_end as usize - ksupport_start as usize);
 //   #[cfg(not(has_emulator))]
    csr::emulator::sdram_cp_from_write(ksupport_start as u32);
    csr::emulator::sdram_cp_to_write(KERNELCPU_EXEC_ADDRESS as u32 - KSUPPORT_HEADER_SIZE as u32);
    csr::emulator::sdram_cp_size_write(ksupport_end as u32 - ksupport_start as u32);
    csr::emulator::sdram_cp_write(1);
    
    csr::kernel_cpu::reset_write(0);
    info!("kernel:start - reset_write(0) - about to start rpc_queue");
    rpc_queue::init();
    info!("rpc_queue::init() completed");
}

#[cfg(not(has_kernel_cpu))]
pub unsafe fn start() {
    unimplemented!("not(has_kernel_cpu)")
}

pub unsafe fn stop() {
    #[cfg(has_kernel_cpu)]
    csr::kernel_cpu::reset_write(1);

    mailbox::acknowledge();
    rpc_queue::init();
}

pub fn validate(ptr: usize) -> bool {
    ptr >= KERNELCPU_EXEC_ADDRESS && ptr <= KERNELCPU_LAST_ADDRESS
}
