use core::cell::RefCell;
use cortex_m::interrupt::{self, Mutex};
use stm32f4xx_hal::{pac, prelude::*, rcc, timer::delay::SysDelay};

/// Static variable for storing an instance of the peripherals.
pub static PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

/// A structure representing a set of microcontroller peripherals.
pub struct Peripherals {
    pub clocks: Option<rcc::Clocks>,
    pub gpioa: Option<pac::GPIOA>,
    pub usart1: Option<pac::USART1>,
    pub tim2: Option<pac::TIM2>,
    pub tim3: Option<pac::TIM3>,
    pub tim4: Option<pac::TIM4>,
    pub tim5: Option<pac::TIM5>,
    pub tim7: Option<pac::TIM7>,
    pub delay: Option<SysDelay>,
}

impl Peripherals {
    /// Creates an instance of the `Peripherals` structure.
    pub fn new() -> Self {
        let dp = pac::Peripherals::take().unwrap();
        let cp = cortex_m::peripheral::Peripherals::take().unwrap();

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        // Create a delay abstraction based on SysTick
        let delay = cp.SYST.delay(&clocks);

        Peripherals {
            clocks: Some(clocks),
            gpioa: Some(dp.GPIOA),
            usart1: Some(dp.USART1),
            tim2: Some(dp.TIM2),
            tim3: Some(dp.TIM3),
            tim4: Some(dp.TIM4),
            tim5: Some(dp.TIM5),
            tim7: Some(dp.TIM7),
            delay: Some(delay),
        }
    }
}

/// Initializes the microcontroller's peripheral devices.
pub fn init_peripherals() {
    interrupt::free(|cs| {
        *PERIPHERALS.borrow(cs).borrow_mut() = Some(Peripherals::new());
    });
}
