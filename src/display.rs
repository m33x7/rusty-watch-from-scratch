use esp_idf_hal::gpio::{self, PinDriver};
use esp_idf_hal::spi::{self, config::{Config, Mode, Phase, Polarity}, SpiDeviceDriver };
use esp_idf_hal::units::FromValueType;
use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::peripherals::Peripherals;

use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, TimeZone};
use chrono_tz::Europe::Berlin;

use gc9a01::{prelude::*, Gc9a01, SPIDisplayInterface};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Point, RgbColor},
    primitives::{PrimitiveStyleBuilder},
    text::{Text},
    mono_font::{ascii::FONT_6X10, ascii::FONT_10X20, MonoTextStyle},
    Drawable,
};

pub struct display_data<'a> {
    pub sck: gpio::Gpio10<'a>,
    pub mosi: gpio::Gpio11<'a>,
    pub cs: gpio::Gpio9<'a>,
    pub dc: gpio::Gpio8<'a>,
    pub reset: gpio::Gpio14<'a>,
    pub backlight: gpio::Gpio2<'a>,
    pub spi2: spi::SPI2<'a>,
}

pub fn display_task<'a>(data: display_data<'a>){
    // Init pins for display
    let dc_output = PinDriver::output(data.dc).unwrap();
    let mut backlight_output = PinDriver::output(data.backlight).unwrap();
    let mut reset_output = PinDriver::output(data.reset).unwrap();

    backlight_output.set_high().unwrap();

    let mut delay = Delay::new_default();

    let driver: spi::SpiDriver<'_> = spi::SpiDriver::new(
        data.spi2,
        data.sck,
        data.mosi,
        None::<gpio::AnyIOPin>,
        &spi::SpiDriverConfig::new(),
    ).unwrap();

    let config = Config::new().baudrate(2.MHz().into()).data_mode(Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    });

    let spi_device = SpiDeviceDriver::new(driver, Some(data.cs), &config).unwrap();
    let interface = SPIDisplayInterface::new(spi_device, dc_output);
    let mut display_driver = Box::new(Gc9a01::new(interface, DisplayResolution240x240, DisplayRotation::Rotate270)).into_buffered_graphics();
    
    display_driver.reset(&mut reset_output, &mut delay).ok();
    display_driver.init(&mut delay).ok();
    log::info!("Driver configured!");

    loop {
        FreeRtos::delay_ms(500);

        let bat_mv = 4500; // Some fake battery voltage for now.
        let text = format!("VBAT: {:?}", bat_mv);

        let time= format_time(current_time_us());

        let _ = display_driver.clear();
        
        let _ = Text::new(&text, Point::new(50, 50), MonoTextStyle::new(&FONT_6X10, Rgb565::RED))
            .draw(&mut display_driver);

        let _ = Text::new(&time, Point::new(80, 130), MonoTextStyle::new(&FONT_10X20, Rgb565::RED))
            .draw(&mut display_driver);

        let _ = display_driver.flush();
    }
}

pub fn current_time_us() -> i64 {
    // After NTP sync, esp-idf's system clock is backed by the RTC timer.
    // SystemTime::now() survives deep sleep correctly.
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    d.as_secs() as i64 * 1_000_000 + d.subsec_micros() as i64
}

pub fn format_time(epoch_us: i64) -> String {
    let epoch_secs = epoch_us / 1_000_000;
    let nanos      = ((epoch_us % 1_000_000) * 1000) as u32;

    let utc: DateTime<Utc> = Utc.timestamp(epoch_secs, nanos);
    let local = utc.with_timezone(&Berlin);

    local.format("%H:%M:%S").to_string()
}