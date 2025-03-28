#![no_std]
#![no_main]

use flash_algorithm::*;
use rtt_target::{rprintln, rtt_init_print};

// Import the `cortex_ar` crate. This is necessary to tell the compiler that we do
// use this crate, even if we don't call any functions from inside it. Without this,
// critical section code would be unavailable, leading to opaque errors such as
// `undefined symbol: _critical_section_1_0_acquire`.
use cortex_ar as _;

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
