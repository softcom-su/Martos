#![no_main]
#![no_std]

use core::fmt::Write; // for pretty formatting of the serial output
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use martos::task_manager::{TaskManager, TaskManagerTrait};
use martos::{get_uart, init_system};

// Counter to work with in loop.
static COUNTER: AtomicU32 = AtomicU32::new(1);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    hprintln!("Panic!");
    loop {}
}

/// Setup function for task to execute.
fn setup_fn() {
    let mut uart = get_uart();

    writeln!(uart, "Setup 'hello world' from UART!\r").unwrap();
}

/// Loop function for task to execute.
fn loop_fn() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
    hprintln!("Counter = {}", unsafe { COUNTER.as_ptr().read() });
}

/// Stop condition function for task to execute.
fn stop_condition_fn() -> bool {
    let value = unsafe { COUNTER.as_ptr().read() };
    if value % 50 == 0 {
        return true;
    }
    return false;
}

#[entry]
fn main() -> ! {
    // Initialize Martos.
    init_system();
    // Add task to execute.
    TaskManager::add_task(setup_fn, loop_fn, stop_condition_fn);
    // Start task manager.
    TaskManager::start_task_manager();
}
