pub mod hardware_timer;
pub mod memory_manager;
pub mod peripherals;
pub mod uart;
use crate::ports::PortTrait;
use stm32f4xx_hal::{pac, serial::Serial};

/// PortTrait implementation for Cortex_m platform
pub struct CortexM;
impl PortTrait for CortexM {
    fn init_heap() {
        memory_manager::init_heap();
    }

    fn setup_hardware_timer() {
        hardware_timer::setup_hardware_timer();
    }

    fn valid_timer_index(timer_index: u8) -> bool {
        (2..=5).contains(&timer_index)
    }

    fn try_acquire_timer(timer_index: u8) -> bool {
        hardware_timer::try_acquire_timer(timer_index)
    }

    fn start_hardware_timer(timer_index: u8) {
        hardware_timer::start_hardware_timer(timer_index);
    }

    fn set_reload_mode(timer_index: u8, auto_reload: bool) {
        hardware_timer::set_reload_mode(timer_index, auto_reload);
    }

    fn change_period_timer(timer_index: u8, period: core::time::Duration) {
        hardware_timer::change_period_timer(timer_index, period);
    }

    fn get_time(timer_index: u8) -> core::time::Duration {
        hardware_timer::get_time(timer_index)
    }

    fn stop_hardware_timer(timer_index: u8) -> bool {
        hardware_timer::stop_hardware_timer(timer_index)
    }

    fn release_hardware_timer(timer_index: u8) {
        hardware_timer::release_hardware_timer(timer_index)
    }

    fn get_uart() -> Serial<pac::USART1> {
        uart::get_uart()
    }
}
