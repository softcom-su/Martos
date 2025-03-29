#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln}; // for debug
use martos::{
    init_system,
    task_manager::{TaskManager, TaskManagerTrait},
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Counter to work with in loop.
static COUNTER: AtomicU32 = AtomicU32::new(1);
/// Vector to work with in loop.
static mut VEC: Vec<u32> = Vec::new();

/// Setup function for task to execute.
fn setup_fn() {
    hprintln!("Setup hello world!")
}

/// Loop function for task to execute.
fn loop_fn() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
    unsafe {
        VEC.push(COUNTER.as_ptr().read());
    }
    hprintln!("Loop hello world!");
    hprintln!("Vector last value = {}", unsafe { VEC.last().unwrap() });
}

/// Stop condition function for task to execute.
fn stop_condition_fn() -> bool {
    let value = unsafe { COUNTER.as_ptr().read() };
    if value % 50 == 0 {
        // exit QEMU
        // NOTE do not run this on hardware; it can corrupt OpenOCD state
        // debug::exit(debug::EXIT_SUCCESS);

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
