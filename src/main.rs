use std::panic;

use esp_idf_hal::i2c;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::peripherals::Peripherals,
    nvs::EspDefaultNvsPartition,
    timer::EspTaskTimerService,
    wifi::{BlockingWifi, EspWifi},
};

use esp_idf_hal::gpio::{self, PinDriver};
use esp_idf_hal::delay::{Delay, FreeRtos};

mod battery;
mod wifi_sntp;
mod touch;
mod display;

fn app_main() -> anyhow::Result<()>{
    let peripherals = Peripherals::take().unwrap();
    let pins: gpio::Pins = peripherals.pins;

    let _timer_service = EspTaskTimerService::new().unwrap();

    // Init pins for touch
    let i2c_sda = pins.gpio6;
    let i2c_scl = pins.gpio7;
    let mut i2c = i2c::I2cDriver::new(peripherals.i2c0, i2c_sda, i2c_scl, &i2c::I2cConfig::new())?;

    // Reading SNTP from WIFI
    let sysloop      = EspSystemEventLoop::take()?;
    let nvs          = EspDefaultNvsPartition::take()?;

    // --- sync clocks over wifi ---
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;
    wifi_sntp::sync_clocks(wifi);

    let display_data = display::display_data {
        sck: pins.gpio10,
        mosi: pins.gpio11,
        cs: pins.gpio9,
        dc: pins.gpio8,
        reset: pins.gpio14,
        backlight: pins.gpio2,
        spi2: peripherals.spi2,
    };

    let display_thread = std::thread::Builder::new()
        .stack_size(7000 + (240 * 240 * 2) + (240 * 12))
        .spawn(move || display::display_task(display_data))?;

    display_thread.join();

    Ok(())

    /*
    let touch_task_data = touch::TouchTaskData { delay, i2c: &mut i2c, int1: pins.gpio5, reset: pins.gpio13 };
    touch::touch_task(touch_task_data);
    */

}

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    panic::set_hook(Box::new(|info| {
        println!("PANIC: {}", info);
    }));

    match app_main() {
        Ok(()) => log::info!("terminated"),
        Err(e) => log::error!("{:?}", e),
    }
}