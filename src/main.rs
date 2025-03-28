#![no_std]
#![no_main]

use f021::{FlashBank, FsmStatus};
use flash_algorithm::{ErrorCode, FlashAlgorithm, Function};
use rtt_target::{rprint, rprintln, rtt_init_print};

// Import the `cortex_ar` crate. This is necessary to tell the compiler that we do
// use this crate, even if we don't call any functions from inside it. Without this,
// critical section code would be unavailable, leading to opaque errors such as
// `undefined symbol: _critical_section_1_0_acquire`.
use cortex_ar as _;

mod f021;

/// Set this to `true` in order to print sector information for inclusion
/// in the `flash_algorithm::algorithm!()` definition.
const PRINT_SECTOR_INFORMATION: bool = false;

/// Set to `true` to print version information on init.
const PRINT_VERSION_INFORMATION: bool = false;

/// The number of bytes that the flash API can take at once.
const WRITE_BLOCK_SIZE: usize = 32;

/// HCLK comes from OSCIN by default, which is a 16 MHz crystal on Launch-XL2
const DEFAULT_CLOCK: u32 = 16;

/// The default EWAIT is fine for 16 MHz clock
const DEFAULT_EWAIT: Option<u32> = None;

/// Read value comes from datasheet
const DEFAULT_RWAIT: Option<u32> = Some(3);

/// When performing a blank check, we limit the amount of data we process at
/// a time in order to make retries less catastrophic.
const BLANK_CHECK_BYTE_COUNT: u32 = 1024;

/// Due to being unable to disable the ECC on the TMS570, we need to try
/// multiple times in the event of a failure.
const BLANK_CHECK_RETRIES: usize = 6;

// A collection of registers

const FBPWRMODE: *mut u32 = 0xfff8_7040 as *mut u32;
const EWAIT: *mut u32 = 0xfff8_72b8 as *mut u32;
const FSM_WR_ENA: *mut u32 = 0xfff8_7288 as *mut u32;
const FRDCNTL: *mut u32 = 0xfff8_7000 as *mut u32;

const SECTORS: &[FlashSector] = &[
    // FlashSector {
    //     address: 0,
    //     size: 0x00200000,
    // },
    // FlashSector {
    //     address: 0x00200000,
    //     size: 0x00200000,
    // },
    // Bank 0
    FlashSector {
        size: 0x4000,
        address: 0x0,
    },
    FlashSector {
        size: 0x4000,
        address: 0x4000,
    },
    FlashSector {
        size: 0x4000,
        address: 0x8000,
    },
    FlashSector {
        size: 0x4000,
        address: 0xc000,
    },
    FlashSector {
        size: 0x4000,
        address: 0x10000,
    },
    FlashSector {
        size: 0x4000,
        address: 0x14000,
    },
    FlashSector {
        size: 0x8000,
        address: 0x18000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x20000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x40000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x60000,
    },
    FlashSector {
        size: 0x40000,
        address: 0x80000,
    },
    FlashSector {
        size: 0x40000,
        address: 0xc0000,
    },
    FlashSector {
        size: 0x40000,
        address: 0x100000,
    },
    FlashSector {
        size: 0x40000,
        address: 0x140000,
    },
    FlashSector {
        size: 0x40000,
        address: 0x180000,
    },
    FlashSector {
        size: 0x40000,
        address: 0x1c0000,
    },
    // Bank 1
    FlashSector {
        size: 0x20000,
        address: 0x200000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x220000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x240000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x260000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x280000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x2a0000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x2c0000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x2e0000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x300000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x320000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x340000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x360000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x380000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x3a0000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x3c0000,
    },
    FlashSector {
        size: 0x20000,
        address: 0x3e0000,
    },
];

struct Algorithm;

fn bank_for_address(address: u32) -> FlashBank {
    if address > 0xf0200000 {
        FlashBank::_7
    } else if address > 0x200000 {
        FlashBank::_1
    } else {
        FlashBank::_0
    }
}

flash_algorithm::algorithm!(Algorithm, {
    device_name: "f021",
    device_type: DeviceType::Onchip,
    flash_address: 0,
    flash_size: (4*1024*1024),
    // Note that the page size is effectively the same as
    // the sector size due to how the API works. The value
    // presented here is simply the smallest sector size.
    page_size: (16*1024),
    empty_value: 0xFFu8,
    program_time_out: 2000,
    erase_time_out: 14000,
    sectors: [
        // Bank 0
        {
            size: 0x4000u32,
            address: 0x0u32,
        },
        {
            size: 0x4000u32,
            address: 0x4000u32,
        },
        {
            size: 0x4000u32,
            address: 0x8000u32,
        },
        {
            size: 0x4000u32,
            address: 0xc000u32,
        },
        {
            size: 0x4000u32,
            address: 0x10000u32,
        },
        {
            size: 0x4000u32,
            address: 0x14000u32,
        },
        {
            size: 0x8000u32,
            address: 0x18000u32,
        },
        {
            size: 0x20000u32,
            address: 0x20000u32,
        },
        {
            size: 0x20000u32,
            address: 0x40000u32,
        },
        {
            size: 0x20000u32,
            address: 0x60000u32,
        },
        {
            size: 0x40000u32,
            address: 0x80000u32,
        },
        {
            size: 0x40000u32,
            address: 0xc0000u32,
        },
        {
            size: 0x40000u32,
            address: 0x100000u32,
        },
        {
            size: 0x40000u32,
            address: 0x140000u32,
        },
        {
            size: 0x40000u32,
            address: 0x180000u32,
        },
        {
            size: 0x40000u32,
            address: 0x1c0000u32,
        },
        // Bank 1
        {
            size: 0x20000u32,
            address: 0x200000u32,
        },
        {
            size: 0x20000u32,
            address: 0x220000u32,
        },
        {
            size: 0x20000u32,
            address: 0x240000u32,
        },
        {
            size: 0x20000u32,
            address: 0x260000u32,
        },
        {
            size: 0x20000u32,
            address: 0x280000u32,
        },
        {
            size: 0x20000u32,
            address: 0x2a0000u32,
        },
        {
            size: 0x20000u32,
            address: 0x2c0000u32,
        },
        {
            size: 0x20000u32,
            address: 0x2e0000u32,
        },
        {
            size: 0x20000u32,
            address: 0x300000u32,
        },
        {
            size: 0x20000u32,
            address: 0x320000u32,
        },
        {
            size: 0x20000u32,
            address: 0x340000u32,
        },
        {
            size: 0x20000u32,
            address: 0x360000u32,
        },
        {
            size: 0x20000u32,
            address: 0x380000u32,
        },
        {
            size: 0x20000u32,
            address: 0x3a0000u32,
        },
        {
            size: 0x20000u32,
            address: 0x3c0000u32,
        },
        {
            size: 0x20000u32,
            address: 0x3e0000u32,
        } // Must skip final `,`
    ]
});

#[inline(never)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", info);
    // Wait for the rprintln to finish
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, mut clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        f021::invalidate_caches();
        rtt_init_print!(rtt_target::ChannelMode::BlockIfFull, 256);

        if clock == 0 {
            rprintln!(
                "Clock was detected as 0 -- setting to {} MHz",
                DEFAULT_CLOCK
            );
            clock = DEFAULT_CLOCK;
        }
        rprintln!("Initializing FAPI...");

        // Ensure the EEPROM is powered up, since that is read by `initialize_flash_banks()`.
        let fbpwrmode = unsafe { FBPWRMODE.read_volatile() };
        if fbpwrmode & 0xffff != 0xffff {
            rprintln!("FBPWRMODE was {:04x} and not 0xffff -- fixing", fbpwrmode);
            unsafe {
                FBPWRMODE.write_volatile(0x0505_ffff);
            }
        }

        if let Some(ewait) = DEFAULT_EWAIT {
            rprintln!("Setting EWAIT to {}", ewait);
            unsafe {
                FSM_WR_ENA.write_volatile(5);
                EWAIT.write_volatile((ewait & 15) << 16);
                FSM_WR_ENA.write_volatile(2);
            }
        }

        if let Some(rwait) = DEFAULT_RWAIT {
            unsafe {
                let frdcntl = FRDCNTL.read_volatile();
                rprintln!("Current RWAIT: {}", (frdcntl >> 8) & 15);
                let frdcntl = (frdcntl & !(15 << 8)) | ((rwait & 15) << 8) | 0b1 | 0b10;
                rprintln!("Setting RWAIT to {}", rwait);
                FRDCNTL.write_volatile(frdcntl);
            }
        }

        rprintln!("Calling initialize_flash_banks({})...", clock);
        if let Err(e) = f021::initialize_flash_banks(clock) {
            rprintln!(
                "Unable to initialize flash bank with clock {}: {}",
                clock,
                e
            );
            return Err(e.into());
        }

        if PRINT_VERSION_INFORMATION {
            rprint!("Getting library info:");
            let library_info = f021::library_info();
            rprintln!("   {:x?}", library_info);

            rprint!("Getting device info: ");
            let device_info = f021::device_info();
            rprintln!("   {:x?}", device_info);
        }

        if PRINT_SECTOR_INFORMATION {
            rprintln!("Flash bank information:");
            for bank_number in 0..8 {
                rprint!("   Bank {}", bank_number);
                match f021::bank_sectors(bank_number.try_into().unwrap()) {
                    Ok(bank_sectors) => rprintln!(
                        " @ {:08x}: {:?}",
                        bank_sectors.bank_start_address,
                        bank_sectors
                    ),
                    Err(e) => rprintln!(": Error - {}", e),
                }
            }
        }

        if PRINT_SECTOR_INFORMATION {
            rprintln!("    sectors: [");
            for bank_number in 0..=1 {
                let bank_sectors = f021::bank_sectors(bank_number.try_into().unwrap()).unwrap();
                let mut start = bank_sectors.bank_start_address;
                rprintln!("        // Bank {}", bank_number);
                for bank_size in bank_sectors.sector_sizes() {
                    rprintln!("        {");
                    rprintln!("            size: 0x{:x},", bank_size);
                    rprintln!("            address: 0x{:x},", start);
                    rprintln!("        },");
                    start += bank_size;
                }
            }
            rprintln!("    ],");
        }

        // rprintln!("Activating flash bank 0");
        // if let Err(e) = f021::set_active_flash_bank(f021::FlashBank::_0) {
        //     rprintln!("Unable to activate flash bank 0: {}", e);
        //     return Err(e.into());
        // }

        // rprintln!("Enabling main bank sectors...");
        // if let Err(e) = f021::enable_main_bank_sectors(u16::MAX) {
        //     rprintln!("Unable to enable main bank sectors: {}", e);
        //     return Err(e.into());
        // }

        // rprintln!("FSM status: {:?}", f021::fsm_status());

        rprintln!("F021 initialized");

        Ok(Self)
    }

    // Value at 0x4000 before: 0xe2801028
    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        // Wait for any existing FSM activity to finish
        while f021::fsm_running() == FsmStatus::Busy {}

        for bank_number in 0..=1 {
            // Wait for any existing FSM activity to finish
            while f021::fsm_running() == FsmStatus::Busy {}

            if let Err(e) = f021::set_active_flash_bank(bank_number.try_into().unwrap()) {
                rprintln!("Unable to set flash bank {}: {}", bank_number, e);
                return Err(e.into());
            }

            if let Err(e) = f021::enable_main_bank_sectors(u16::MAX) {
                rprintln!(
                    "Unable to enable main bank sectors for bank {}: {}",
                    bank_number,
                    e
                );
                return Err(e.into());
            }
            if let Err(e) = f021::issue_async_command_with_address(
                f021::FlashStateCommand::EraseBank,
                core::ptr::null_mut(),
            ) {
                rprintln!("Unable to erase bank {}: {}", bank_number, e);
            }
            while f021::fsm_running() == FsmStatus::Busy {}
        }

        f021::flush();

        Ok(())
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        // Wait for any existing FSM activity to finish
        while f021::fsm_running() == FsmStatus::Busy {}

        for sector in SECTORS {
            if addr < sector.address || addr >= sector.address + sector.size {
                continue;
            }
            let bank_number = bank_for_address(addr);

            rprintln!(
                "Bank {:?}, Sector {{ address: 0x{:08x}, size: {} }}",
                bank_number,
                sector.address,
                sector.size
            );

            // Wait for any existing FSM activity to finish
            while f021::fsm_running() == FsmStatus::Busy {}

            if let Err(e) = f021::set_active_flash_bank(bank_number) {
                rprintln!("Unable to set flash bank {:?}: {}", bank_number, e);
                return Err(e.into());
            }

            if let Err(e) = f021::enable_main_bank_sectors(u16::MAX) {
                rprintln!(
                    "Unable to enable main bank sectors for bank {:?}: {}",
                    bank_number,
                    e
                );
                return Err(e.into());
            }

            if let Err(e) = f021::issue_async_command(f021::FlashStateCommand::ClearStatus) {
                rprintln!("Unable to clear status: {}", e);
            }

            if let Err(e) = f021::issue_async_command_with_address(
                f021::FlashStateCommand::EraseSector,
                addr as *mut u32,
            ) {
                rprintln!("Unable to erase sector: {}", e);
            }

            while f021::fsm_running() == FsmStatus::Busy {}

            f021::flush();

            // Run a blank check just to be sure.
            self.blank_check(sector.address, sector.size)?;

            rprintln!("Sector erased");
            return Ok(());
        }

        rprintln!(
            "Unable to erase sector addr {:08x} -- couldn't find sector information",
            addr
        );
        Err(ErrorCode::new(4).unwrap())
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        rprintln!("Program Page addr: 0x{:08x} size:{}", addr, data.len());
        // Wait for any existing FSM activity to finish
        while f021::fsm_running() == FsmStatus::Busy {}

        let bank_number = bank_for_address(addr);

        // Wait for any existing FSM activity to finish
        while f021::fsm_running() == FsmStatus::Busy {}

        if let Err(e) = f021::set_active_flash_bank(bank_number) {
            rprintln!("Unable to set flash bank {:?}: {}", bank_number, e);
            return Err(e.into());
        }

        if let Err(e) = f021::enable_main_bank_sectors(u16::MAX) {
            rprintln!(
                "Unable to enable main bank sectors for bank {:?}: {}",
                bank_number,
                e
            );
            return Err(e.into());
        }

        for (offset, bytes) in data.chunks(WRITE_BLOCK_SIZE).enumerate() {
            if let Err(e) = f021::issue_programming_command(
                (addr + (offset * WRITE_BLOCK_SIZE) as u32) as *mut u32,
                bytes,
                None,
                f021::FlashProgrammingCommand::AutoEccGeneration,
            ) {
                rprintln!("Unable to program sector: {}", e);
            }
            while f021::fsm_running() == FsmStatus::Busy {}
        }

        f021::flush();

        Ok(())
    }

    fn blank_check(&mut self, mut address: u32, mut size: u32) -> Result<(), ErrorCode> {
        // Run a blank check. OR a `1` into the resulting address in case address
        // 0 is not blank.
        while f021::fsm_running() == FsmStatus::Busy {}

        if address & (BLANK_CHECK_BYTE_COUNT - 1) != 0 {
            rprintln!(
                "Error: Blank check must occur on an address aligned to {} bytes",
                BLANK_CHECK_BYTE_COUNT
            );
            return Err(ErrorCode::new(2).unwrap());
        }

        while size > 0 {
            // Check in chunks of BLANK_CHECK_BYTE_COUNT bytes.
            let to_check = size.min(BLANK_CHECK_BYTE_COUNT);

            // Check multiple times. Strictly speaking, the "blank check" function is
            // not supposed to be used on banks 0 or 1 because an erased flash contains
            // errors, and those errors will sometimes get corrected by the machinery,
            // resulting in blank check failures. However, performing the check multiple
            // times appears to work.
            //
            // See https://e2e.ti.com/support/microcontrollers/arm-based-microcontrollers-group/arm-based-microcontrollers/f/arm-based-microcontrollers-forum/1116947/tms570lc4357-sw-stuck-in-fapi_doblankcheck-when-trying-to-update-fee-in-case-of-freertos/4139678#4139678
            //
            //  > Hi Sakti,
            //  > Fapi_doBlankCheck() is to check the erase state of flash bank. As the erase
            //  > state of the Flash is not a valid ECC condition, the ECC check and correction
            //  > must be disabled. But the flash ECC on TMS570LC43x is enabled by default and
            //  > can not be disabled. We don't suggest using this function in your project.
            let mut check_passed = false;
            for try_number in 0..BLANK_CHECK_RETRIES {
                if let Err(e) = f021::blank_check(address, to_check) {
                    rprintln!(
                        "Blank check error (try {}/{}): {:x?}",
                        try_number + 1,
                        BLANK_CHECK_RETRIES,
                        e
                    );
                    continue;
                }
                check_passed = true;
                break;
            }
            if !check_passed {
                rprintln!("Check failed after {} tries", BLANK_CHECK_RETRIES);
                return Err(ErrorCode::new(6).unwrap());
            }
            address += to_check;
            size -= to_check;
        }
        Ok(())
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        // TODO: Add code here to uninitialize the flash algorithm.
    }
}
