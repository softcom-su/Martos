use crate::ports::xtensa_esp32::hardware_timer::{
    PERIFERALS_RADIO_CLK, PERIFERALS_RNG, PERIFERALS_WIFI, TIMER10,
};
use esp_hal::rng::Rng;
use esp_wifi::{esp_now::EspNow, init};

pub static mut ESP_NOW: Option<EspNow> = None;
static mut INIT: Option<esp_wifi::EspWifiController> = None;

/// Network initialization.
pub fn init_network() {
    unsafe {
        let peripherals_rng = PERIFERALS_RNG.take().expect("RNG peripherals error");
        let peripherals_radio_clk = PERIFERALS_RADIO_CLK
            .take()
            .expect("RADIO_CLK peripherals error");
        let timer10 = TIMER10.take().expect("Network timer error");
        let periferals_wifi = PERIFERALS_WIFI.take().expect("WIFI peripherals error");

        INIT = Some(init(timer10, Rng::new(peripherals_rng), peripherals_radio_clk).unwrap());
        let init_ref = INIT.as_ref().unwrap();

        ESP_NOW = Some(esp_wifi::esp_now::EspNow::new(init_ref, periferals_wifi).unwrap());
    }
}

/// Getting esp-now object for network.
pub fn get_esp_now() -> EspNow<'static> {
    unsafe {
        let esp_now = ESP_NOW.take().expect("Esp-now error");
        return esp_now;
    }
}
