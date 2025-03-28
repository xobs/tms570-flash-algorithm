#![no_std]
#![no_main]

#[no_mangle]
pub fn _critical_section_1_0_acquire() -> bool {
    use core::sync::atomic;
    // the i bit means "masked"
    let was_active = !cortex_ar::register::Cpsr::read().i();
    cortex_ar::interrupt::disable();
    atomic::compiler_fence(atomic::Ordering::SeqCst);
    was_active
}

#[no_mangle]
pub fn _critical_section_1_0_release(was_active: bool) {
    use core::sync::atomic;
    // Only re-enable interrupts if they were enabled before the critical section.
    if was_active {
        atomic::compiler_fence(atomic::Ordering::SeqCst);
        // Safety: This is OK because we're releasing a lock that was
        // entered with interrupts enabled
        unsafe {
            cortex_ar::interrupt::enable();
        }
    }
}

use flash_algorithm::*;
use rtt_target::{rprintln, rtt_init_print};

struct Algorithm;

algorithm!(Algorithm, {
    flash_address: 0,
    flash_size: 524288,
    page_size: 0x400,
    empty_value: 0xFF,
    sectors: [{
        size: 0x400,
        address: 0x0,
    }]
});

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        rtt_init_print!();
        rprintln!("Init");
        // TODO: Add setup code for the flash algorithm.
        Ok(Self)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        rprintln!("Erase All");
        // TODO: Add code here that erases the entire flash.
        Err(ErrorCode::new(0x70d0).unwrap())
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        rprintln!("Erase sector addr:{}", addr);
        // TODO: Add code here that erases a page to flash.
        Ok(())
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        rprintln!("Program Page addr:{} size:{}", addr, data.len());
        // TODO: Add code here that writes a page to flash.
        Ok(())
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        // TODO: Add code here to uninitialize the flash algorithm.
    }
}
