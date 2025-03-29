pub mod memory_manager;
pub mod uart;
use crate::ports::PortTrait;
use core::time::Duration;
use stm32f4xx_hal::{pac, serial::Serial};

/// PortTrait implementation for Cortex_m platform
pub struct CortexM;
impl PortTrait for CortexM {
    fn init_heap() {
        memory_manager::init_heap();
    }

    fn setup_hardware_timer() {}

    fn valid_timer_index(_timer_index: u8) -> bool {
        true
    }

    fn try_acquire_timer(_timer_index: u8) -> bool {
        false
    }

    fn start_hardware_timer(_timer_index: u8) {}

    fn set_reload_mode(_timer_index: u8, _auto_reload: bool) {}

    fn change_period_timer(_timer_index: u8, _period: core::time::Duration) {}

    fn get_time(_timer_index: u8) -> core::time::Duration {
        Duration::new(1, 0) // dummy
    }

    fn stop_hardware_timer(_timer_index: u8) -> bool {
        false
    }

    fn release_hardware_timer(_timer_index: u8) {}

    fn get_uart() -> Serial<pac::USART1> {
        uart::get_uart()
    }
}
