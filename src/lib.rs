#![no_std]
extern crate alloc;

mod ports;
use ports::PortTrait;
#[cfg(feature = "c-library")]
pub mod c_api;
pub mod task_manager;
pub mod timer;
#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
#[cfg(feature = "network")]
use esp_wifi::esp_now::EspNow;

#[cfg(all(target_arch = "arm", feature = "cortex_m"))]
use stm32f4xx_hal::{pac, serial::Serial};

/// Martos initialization. Should be called before using Martos functions.
pub fn init_system() {
    // Memory initialization.
    ports::Port::init_heap();

    #[cfg(all(target_arch = "arm", feature = "cortex_m"))]
    // Peripherals setup
    ports::cortex_m::peripherals::init_peripherals();

    // Hardware timer setup.
    ports::Port::setup_hardware_timer();
    #[cfg(feature = "network")]
    // Network setup.
    ports::Port::init_network();
}

#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
#[cfg(feature = "network")]
pub fn get_esp_now() -> EspNow<'static> {
    return ports::Port::get_esp_now();
}

#[cfg(all(target_arch = "arm", feature = "cortex_m"))]
pub fn get_uart() -> Serial<pac::USART3> {
    return ports::Port::get_uart();
}

#[cfg(all(target_arch = "arm", feature = "cortex_m"))]
pub fn delay(time: core::time::Duration) {
    ports::Port::delay(time);
}
