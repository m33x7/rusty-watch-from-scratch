use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::adc::attenuation::DB_11;
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::oneshot::{AdcChannelDriver, AdcDriver};
use esp_idf_hal::adc::ADC1;
use esp_idf_hal::gpio::Gpio1;

use std::sync::Arc;
use crate::state;

pub struct BatteryMonitorData<'a> {
    pub adc1: ADC1<'a>,
    pub gpio1: Gpio1<'a>,
    pub state: Arc<state::State>
}

pub fn battery_monitor_task<'a>(data: BatteryMonitorData<'a>){
    let adc = AdcDriver::new(data.adc1).expect("Failed to init ADC");
    let mut pin = AdcChannelDriver::new(
        &adc,
        data.gpio1,
        &AdcChannelConfig { attenuation: DB_11, ..Default::default() },
    )
    .expect("Failed to init ADC pin");

    loop {
        match pin.read() {
            Ok(mv) => {
                data.state.set_battery_mv(mv * 3);
            }
            Err(e) => {
                log::error!("ADC read error: {}", e);
            }
        }

        FreeRtos::delay_ms(2000);
    }
}