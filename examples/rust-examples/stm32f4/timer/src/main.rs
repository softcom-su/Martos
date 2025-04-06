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
    timer::Timer,
};

/// Counter to work with in loop.
static COUNTER: AtomicU32 = AtomicU32::new(1);
/// Vector to work with in loop.
static mut VEC: Vec<u64> = Vec::new();

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    hprintln!("Panic!");
    loop {}
}

/// Setup function for task to execute.
fn setup_fn() {
    hprintln!("Setup hello world!")
}

/// Loop function for task to execute.
fn loop_fn() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
    let timer2 = Timer::get_timer(2).expect("The timer is busy");
    timer2.change_period_timer(core::time::Duration::from_secs(10));
    timer2.start_timer();
    let time = timer2.get_time();
    timer2.stop_condition_timer();
    timer2.release_timer();
    unsafe {
        VEC.push(time.as_secs() * 1_000_000 + time.subsec_micros() as u64);
    }
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
