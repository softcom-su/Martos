use crate::ports::cortex_m::peripherals::PERIPHERALS;
use cortex_m::interrupt;
use stm32f4xx_hal::{pac, prelude::*, serial::Serial};

/// Cortex_m gets UART1.
pub fn get_uart() -> Serial<pac::USART1> {
    interrupt::free(|cs| {
        let mut peripherals = PERIPHERALS.borrow(cs).borrow_mut();
        let dp = peripherals.as_mut().unwrap();

        let clocks = dp.clocks.take().unwrap();
        let gpioa = dp.gpioa.take().unwrap().split();

        // define RX/TX pins
        let tx_pin = gpioa.pa9.into_alternate();
        let rx_pin = gpioa.pa10.into_alternate();

        let usart1 = dp.usart1.take().unwrap();
        let serial = Serial::new(usart1, (tx_pin, rx_pin), 115200.bps(), &clocks).unwrap();

        dp.clocks.replace(clocks);
        // How to return the remaining pins to peripherals? (It is possible to store each pin separately in the peripherals)
        // dp.gpioa.replace(gpioa);

        serial
    })
}
