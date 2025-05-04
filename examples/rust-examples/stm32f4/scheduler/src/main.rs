#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use martos::{
    init_system,
    task_manager::{TaskManager, TaskManagerTrait},
};

/// Counter to work with in loop.
static COUNTER: AtomicU32 = AtomicU32::new(1);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    hprintln!("Panic!");
    loop {}
}

/// Loop function for task to execute.
fn loop_fn_1() {
    let old = COUNTER.fetch_add(1, Ordering::Relaxed);
    hprintln!("Loop 0; Counter = {}", old);
    cortex_m::asm::delay(50_000_000);
}

fn loop_fn_2() {
    let old = COUNTER.fetch_add(1, Ordering::Relaxed);
    hprintln!("Loop 1; Counter = {}", old);
    cortex_m::asm::delay(50_000_000);
}

fn setup() {
    hprintln!("Setup")
}

fn stop() -> bool {
    hprintln!("Stop");
    if COUNTER.fetch_add(0, Ordering::Relaxed) > 20 {
        true
    } else {
        false
    }
}

#[entry]
fn main() -> ! {
    // Initialize Martos.
    init_system();
    // Add task to execute.
    TaskManager::add_task(setup, loop_fn_1, stop);
    TaskManager::add_task(setup, loop_fn_2, stop);
    // Start task manager.
    TaskManager::start_task_manager();
}
