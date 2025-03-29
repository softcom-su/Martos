#![no_main]
#![no_std]

use cortex_m_rt::entry;
// use cortex_m_semihosting::debug; for debug in QEMU
use martos::get_uart;
use martos::init_system;

use core::fmt::Write; // for pretty formatting of the serial output

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    // Initialize Martos.
    init_system();

    let mut uart = get_uart();

    writeln!(uart, "Hello, world\r").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
