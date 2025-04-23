pub mod hardware_timer;
pub mod memory_manager;
pub mod peripherals;
pub mod uart;
use crate::ports::PortTrait;
use stm32f4xx_hal::{pac, serial::Serial};

#[cfg(feature = "preemptive")]
mod preempt;

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

    fn delay(time: core::time::Duration) {
        hardware_timer::delay(time);
    }

    #[cfg(feature = "preemptive")]
    fn setup_interrupt() {
        preempt::setup_interrupt();
    }

    #[cfg(feature = "preemptive")]
    fn setup_stack(thread: &mut crate::task_manager::preemptive::Thread) {
        preempt::setup_stack(thread);
    }

    #[cfg(feature = "preemptive")]
    fn save_ctx(thread_ctx: &mut TrapFrame, isr_ctx: &TrapFrame) {
        preempt::save_ctx(thread_ctx, isr_ctx)
    }

    #[cfg(feature = "preemptive")]
    fn load_ctx(thread_ctx: &TrapFrame, isr_ctx: &mut TrapFrame) {
        preempt::load_ctx(thread_ctx, isr_ctx)
    }
}

#[cfg(feature = "preemptive")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrapFrame {
    pub r4: u32,  // R4
    pub r5: u32,  // R5
    pub r6: u32,  // R6
    pub r7: u32,  // R7
    pub r8: u32,  // R8
    pub r9: u32,  // R9
    pub r10: u32, // R10
    pub r11: u32, // R11
    pub psp: u32, // Process Stack Pointer
}

#[cfg(feature = "preemptive")]
impl Default for TrapFrame {
    fn default() -> Self {
        TrapFrame {
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            psp: 0,
        }
    }
}
