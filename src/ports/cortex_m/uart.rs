use stm32f4xx_hal::{pac, prelude::*, serial::Serial};

/// Cortex_m gets UART1.
pub fn get_uart() -> Serial<pac::USART1> {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    // define RX/TX pins
    let tx_pin = gpioa.pa9;
    let rx_pin = gpioa.pa10;

    Serial::new(dp.USART1, (tx_pin, rx_pin), 9600.bps(), &clocks).unwrap()
}
