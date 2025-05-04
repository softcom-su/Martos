use crate::ports::cortex_m::peripherals::PERIPHERALS;
use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;
use cortex_m::interrupt::{self, Mutex};
use stm32f4xx_hal::{pac, prelude::*, timer::CounterMs};

const COUNT_OF_TIMERS: usize = 4;

/// Static variable for storing an instance of the timer block.
static TIMER_BLOCK: Mutex<RefCell<Option<TimerBlock>>> = Mutex::new(RefCell::new(None));

/// Structure representing a block of general-purpose timers.
struct TimerBlock {
    /// Timer 2 (contains a 32-bit auto-reload counter).
    timer2: CounterMs<pac::TIM2>,
    /// Timer 3 (contains a 16-bit auto-reload counter).
    timer3: CounterMs<pac::TIM3>,
    /// Timer 4 (contains a 16-bit auto-reload counter).
    timer4: CounterMs<pac::TIM4>,
    /// Timer 5 (contains a 32-bit auto-reload counter).
    timer5: CounterMs<pac::TIM5>,
    /// An indicators showing whether the timers is in use.
    acquired: [AtomicBool; COUNT_OF_TIMERS],
    /// The passed values for the counters.
    duration: [Duration; COUNT_OF_TIMERS],
}

impl TimerBlock {
    /// Creates a new timer block and initializes each timer.
    pub fn setup_hardware_timer() -> Self {
        interrupt::free(|cs| {
            let mut peripherals = PERIPHERALS.borrow(cs).borrow_mut();
            let dp = peripherals.as_mut().unwrap();

            let clocks = dp.clocks.take().unwrap();
            let tim2 = dp.tim2.take().unwrap();
            let tim3 = dp.tim3.take().unwrap();
            let tim4 = dp.tim4.take().unwrap();
            let tim5 = dp.tim5.take().unwrap();

            let timer_block = Self {
                timer2: tim2.counter_ms(&clocks),
                timer3: tim3.counter_ms(&clocks),
                timer4: tim4.counter_ms(&clocks),
                timer5: tim5.counter_ms(&clocks),
                acquired: [const { AtomicBool::new(false) }; COUNT_OF_TIMERS],
                duration: [Duration::new(0, 0); COUNT_OF_TIMERS],
            };

            dp.clocks.replace(clocks);
            timer_block
        })
    }
}

/// Cortex_m hardware timer setup.
pub fn setup_hardware_timer() {
    interrupt::free(|cs| {
        *TIMER_BLOCK.borrow(cs).borrow_mut() = Some(TimerBlock::setup_hardware_timer());
    })
}

/// Cortex_m attempt to acquire timer.
pub fn try_acquire_timer(timer_index: u8) -> bool {
    interrupt::free(|cs| {
        if let Some(timer_block) = TIMER_BLOCK.borrow(cs).borrow_mut().as_mut() {
            if (timer_index - 2) < COUNT_OF_TIMERS as u8
                && !timer_block.acquired[(timer_index - 2) as usize].load(Ordering::Relaxed)
            {
                return timer_block.acquired[(timer_index - 2) as usize]
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok();
            }
        }
        false
    })
}

/// Cortex_m start harware timer.
pub fn start_hardware_timer(timer_index: u8) {
    interrupt::free(|cs| {
        if let Some(timer_block) = TIMER_BLOCK.borrow(cs).borrow_mut().as_mut() {
            match timer_index {
                2 => timer_block
                    .timer2
                    .start(((timer_block.duration[0]).as_micros() as u32).micros())
                    .unwrap(),
                3 => timer_block
                    .timer3
                    .start(((timer_block.duration[1]).as_micros() as u32).micros())
                    .unwrap(),
                4 => timer_block
                    .timer4
                    .start(((timer_block.duration[2]).as_micros() as u32).micros())
                    .unwrap(),
                5 => timer_block
                    .timer5
                    .start(((timer_block.duration[3]).as_micros() as u32).micros())
                    .unwrap(),
                _ => (),
            }
        }
    });
}

/// Cortex_m change operating mode of hardware timer.
/// HAL does not provide the ability to change the timer operating mode, only auto reload mode.
pub fn set_reload_mode(_timer_index: u8, _auto_reload: bool) {}

/// Cortex_m change the period of hardware timer.
pub fn change_period_timer(timer_index: u8, period: Duration) {
    interrupt::free(|cs| {
        if let Some(timer_block) = TIMER_BLOCK.borrow(cs).borrow_mut().as_mut() {
            timer_block.duration[(timer_index - 2) as usize] = period;
        }
    });
}

/// Cortex_m getting counter value of hardware timer.
pub fn get_time(timer_index: u8) -> Duration {
    interrupt::free(|cs| {
        if let Some(timer_block) = TIMER_BLOCK.borrow(cs).borrow_mut().as_ref() {
            let time = match timer_index {
                2 => timer_block.timer2.now().duration_since_epoch().to_micros(),
                3 => timer_block.timer3.now().duration_since_epoch().to_micros(),
                4 => timer_block.timer4.now().duration_since_epoch().to_micros(),
                5 => timer_block.timer5.now().duration_since_epoch().to_micros(),
                _ => 0,
            };
            return Duration::from_micros(time as u64);
        }
        return Duration::new(0, 0);
    })
}

/// Cortex_m stop hardware timer.
pub fn stop_hardware_timer(timer_index: u8) -> bool {
    interrupt::free(|cs| {
        if let Some(timer_block) = TIMER_BLOCK.borrow(cs).borrow_mut().as_mut() {
            match timer_index {
                2 => timer_block.timer2.cancel().unwrap(),
                3 => timer_block.timer3.cancel().unwrap(),
                4 => timer_block.timer4.cancel().unwrap(),
                5 => timer_block.timer5.cancel().unwrap(),
                _ => (),
            }
            return true;
        }
        false
    })
}

/// Cortex_m release hardware timer.
pub fn release_hardware_timer(timer_index: u8) {
    interrupt::free(|cs| {
        if let Some(timer_block) = TIMER_BLOCK.borrow(cs).borrow_mut().as_mut() {
            timer_block.acquired[(timer_index - 2) as usize].store(false, Ordering::Release);
        }
    });
}
