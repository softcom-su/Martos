use crate::ports::cortex_m::{peripherals::PERIPHERALS, TrapFrame};
use crate::task_manager::preemptive::Thread;
use core::arch::asm;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use stm32f4xx_hal::{
    pac::{self, interrupt},
    prelude::*,
    timer::{CounterMs, Event},
};

const TIME_SLICE_MILLIS: u32 = 1000;
static TIM: Mutex<RefCell<Option<CounterMs<pac::TIM7>>>> = Mutex::new(RefCell::new(None));

pub fn setup_interrupt() {
    cortex_m::interrupt::free(|cs| {
        let mut peripherals = PERIPHERALS.borrow(cs).borrow_mut();
        let dp = peripherals.as_mut().unwrap();

        let clocks = dp.clocks.take().unwrap();
        let tim7 = dp.tim7.take().unwrap();
        let mut timer = tim7.counter_ms(&clocks);

        timer.start(TIME_SLICE_MILLIS.millis()).unwrap();
        timer.listen(Event::Update);
        unsafe {
            cortex_m::peripheral::NVIC::unmask(interrupt::TIM7);
        }

        TIM.borrow(cs).replace(Some(timer));
        dp.clocks.replace(clocks);
    });
}

#[interrupt]
fn TIM7() {
    let mut isr_ctx = TrapFrame::default(); // Unnecessary variable

    crate::task_manager::preemptive::PreemptiveTaskManager::schedule(&mut isr_ctx);

    cortex_m::interrupt::free(|cs| {
        let mut timer = TIM.borrow(cs).borrow_mut();
        timer
            .as_mut()
            .unwrap()
            .start(TIME_SLICE_MILLIS.millis())
            .unwrap();
        timer.as_mut().unwrap().listen(Event::Update);
    });

    unsafe {
        asm!("mov lr, #0xfffffffd", "bx lr");
    };
}

pub fn setup_stack(thread: &mut crate::task_manager::preemptive::Thread) {
    // Calculate the top of the stack
    let stack_bottom =
        thread.stack as u32 + crate::task_manager::preemptive::THREAD_STACK_SIZE as u32;

    thread.context.r4 = 0;
    thread.context.r5 = 0;
    thread.context.r6 = 0;
    thread.context.r7 = 0;
    thread.context.r8 = 0;
    thread.context.r9 = 0;
    thread.context.r10 = 0;
    thread.context.r11 = 0;

    thread.context.s16 = 0;
    thread.context.s17 = 0;
    thread.context.s18 = 0;
    thread.context.s19 = 0;
    thread.context.s20 = 0;
    thread.context.s21 = 0;
    thread.context.s22 = 0;
    thread.context.s23 = 0;
    thread.context.s24 = 0;
    thread.context.s25 = 0;
    thread.context.s26 = 0;
    thread.context.s27 = 0;
    thread.context.s28 = 0;
    thread.context.s29 = 0;
    thread.context.s30 = 0;
    thread.context.s31 = 0;

    unsafe {
        // Stack Initialization
        *((stack_bottom - 4) as *mut u32) = 0; // FPSCR
        *((stack_bottom - 8) as *mut u32) = 0; // S15
        *((stack_bottom - 12) as *mut u32) = 0; // S14
        *((stack_bottom - 16) as *mut u32) = 0; // S13
        *((stack_bottom - 20) as *mut u32) = 0; // S12
        *((stack_bottom - 24) as *mut u32) = 0; // S11
        *((stack_bottom - 28) as *mut u32) = 0; // S10
        *((stack_bottom - 32) as *mut u32) = 0; // S9
        *((stack_bottom - 36) as *mut u32) = 0; // S8
        *((stack_bottom - 40) as *mut u32) = 0; // S7
        *((stack_bottom - 44) as *mut u32) = 0; // S6
        *((stack_bottom - 48) as *mut u32) = 0; // S5
        *((stack_bottom - 52) as *mut u32) = 0; // S4
        *((stack_bottom - 56) as *mut u32) = 0; // S3
        *((stack_bottom - 60) as *mut u32) = 0; // S2
        *((stack_bottom - 64) as *mut u32) = 0; // S1
        *((stack_bottom - 68) as *mut u32) = 0; // S0

        *((stack_bottom - 72) as *mut u32) = 0x01000000; // xPSR (Thumb mode)
        *((stack_bottom - 76) as *mut u32) = Thread::run_task as u32; // PC
        *((stack_bottom - 80) as *mut u32) = 0; // LR
        *((stack_bottom - 84) as *mut u32) = 0; // R12
        *((stack_bottom - 88) as *mut u32) = 0; // R3
        *((stack_bottom - 92) as *mut u32) = thread.task.stop_condition_fn as u32; // R2
        *((stack_bottom - 96) as *mut u32) = thread.task.loop_fn as u32; // R1
        *((stack_bottom - 100) as *mut u32) = thread.task.setup_fn as u32; // R0
    };

    thread.context.psp = stack_bottom - 100;
}

pub fn save_ctx(thread_ctx: &mut TrapFrame, _isr_ctx: &TrapFrame) {
    // Save PSP
    thread_ctx.psp = cortex_m::register::psp::read();

    unsafe {
        asm!(
            /* Push non-hardware-stacked registers into Process struct's regs field */
            /* Save registers r4-r11 */
            "str r4, [{0}, #0]",
            "str r5, [{0}, #4]",
            "str r6, [{0}, #8]",
            "str r7, [{0}, #12]",
            "str r8, [{0}, #16]",
            "str r9, [{0}, #20]",
            "str r10, [{0}, #24]",
            "str r11, [{0}, #28]",

            /* Save registers s16-s31 */
            "vstr s16, [{0}, #36]",
            "vstr s17, [{0}, #40]",
            "vstr s18, [{0}, #44]",
            "vstr s19, [{0}, #48]",
            "vstr s20, [{0}, #52]",
            "vstr s21, [{0}, #56]",
            "vstr s22, [{0}, #60]",
            "vstr s23, [{0}, #64]",
            "vstr s24, [{0}, #68]",
            "vstr s25, [{0}, #72]",
            "vstr s26, [{0}, #76]",
            "vstr s27, [{0}, #80]",
            "vstr s28, [{0}, #84]",
            "vstr s29, [{0}, #88]",
            "vstr s30, [{0}, #92]",
            "vstr s31, [{0}, #96]",

            in(reg) thread_ctx as *mut TrapFrame,
            options(nostack)
        );
    };
}

pub fn load_ctx(thread_ctx: &TrapFrame, _isr_ctx: &mut TrapFrame) {
    unsafe {
        asm!(
            /* Restore registers r4-r11 */
            "ldr r4, [{0}, #0]",
            "ldr r5, [{0}, #4]",
            "ldr r6, [{0}, #8]",
            "ldr r7, [{0}, #12]",
            "ldr r8, [{0}, #16]",
            "ldr r9, [{0}, #20]",
            "ldr r10, [{0}, #24]",
            "ldr r11, [{0}, #28]",

            /* Restore registers s16-s31 */
            "vldr s16, [{0}, #36]",
            "vldr s17, [{0}, #40]",
            "vldr s18, [{0}, #44]",
            "vldr s19, [{0}, #48]",
            "vldr s20, [{0}, #52]",
            "vldr s21, [{0}, #56]",
            "vldr s22, [{0}, #60]",
            "vldr s23, [{0}, #64]",
            "vldr s24, [{0}, #68]",
            "vldr s25, [{0}, #72]",
            "vldr s26, [{0}, #76]",
            "vldr s27, [{0}, #80]",
            "vldr s28, [{0}, #84]",
            "vldr s29, [{0}, #88]",
            "vldr s30, [{0}, #92]",
            "vldr s31, [{0}, #96]",

            /* Restore psp */
            "ldr r0, [{0}, #32]",
            "msr psp, r0",

            in(reg) thread_ctx as *const TrapFrame,
            options(nostack)
        );
    }
}
