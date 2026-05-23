use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::adc::attenuation::DB_11;
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::oneshot::{AdcChannelDriver, AdcDriver};
use esp_idf_hal::adc::ADC1;
use esp_idf_hal::gpio::Gpio1;

pub struct BatteryMonitor {
    rx: Receiver<u16>,
}

impl BatteryMonitor {
    pub fn start(adc1: ADC1, gpio1: Gpio1, delay: u32, tx: Sender<u16>) {
        let adc = AdcDriver::new(adc1).expect("Failed to init ADC");
        let mut pin = AdcChannelDriver::new(
            &adc,
            gpio1,
            &AdcChannelConfig {
                attenuation: DB_11,
                ..Default::default()
            },
        )
        .expect("Failed to init ADC pin");

        loop {
            // TODO - provide it as a delay source in function signature.
            FreeRtos::delay_ms(500);

            match pin.read() {
                Ok(raw) => {
                    let mv = (raw as u32 * 3300 / 4095) as u16;
                    tx.send(mv).unwrap(); // No need to handle error here
                }
                Err(e) => {
                    log::error!("ADC read error: {}", e);
                }
            }
        }
    }
}