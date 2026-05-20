use std::panic;
use anyhow::Result;

use esp_idf_hal::i2c;
use esp_idf_svc::{ eventloop::EspSystemEventLoop, hal::peripherals::Peripherals, timer::EspTaskTimerService };

use esp_idf_hal::gpio::{self, PinDriver, Pull};
use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::spi::{self, config::{Config, Mode, Phase, Polarity}, SpiDeviceDriver };
use esp_idf_hal::units::FromValueType;

use display_interface_spi::SPIInterface;
use gc9a01::{prelude::*, Gc9a01, SPIDisplayInterface};

use cst816s::{TouchEvent, TouchGesture, CST816S};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Point, RgbColor, Size},
    primitives::{Circle, Primitive, PrimitiveStyleBuilder, Rectangle},
    text::{Text},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    Drawable,
};

mod battery;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    panic::set_hook(Box::new(|info| {
        println!("PANIC: {}", info);
    }));

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let _sysloop = EspSystemEventLoop::take().unwrap();
    let _timer_service = EspTaskTimerService::new().unwrap();

    // Init pins for display
    let sck = pins.gpio10;
    let mosi = pins.gpio11;
    let cs = pins.gpio9;
    let dc = pins.gpio8;
    let reset = pins.gpio14;
    let backlight = pins.gpio2;
    let cs_output = cs;
    let dc_output = PinDriver::output(dc).unwrap();
    let mut backlight_output = PinDriver::output(backlight).unwrap();
    let mut reset_output = PinDriver::output(reset).unwrap();

    // Init pins for touch
    let i2c_sda = pins.gpio6;
    let i2c_scl = pins.gpio7;
    let cst816s_int1 = PinDriver::input(pins.gpio5, Pull::Up).unwrap();
    let cst816s_reset = PinDriver::output(pins.gpio13).unwrap();
    let i2c = i2c::I2cDriver::new(peripherals.i2c0, i2c_sda, i2c_scl, &i2c::I2cConfig::new())?;

    let mut delay = Delay::new_default();

    backlight_output.set_high().unwrap();

    let driver = spi::SpiDriver::new(
        peripherals.spi2,
        sck,
        mosi,
        None::<gpio::AnyIOPin>,
        &spi::SpiDriverConfig::new(),
    ).unwrap();

    let config = Config::new().baudrate(2.MHz().into()).data_mode(Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    });

    let spi_device = SpiDeviceDriver::new(driver, Some(cs_output), &config).unwrap();
    let interface = SPIDisplayInterface::new(spi_device, dc_output);
    let mut display_driver = Box::new(Gc9a01::new(interface, DisplayResolution240x240, DisplayRotation::Rotate180)).into_buffered_graphics();
    
    display_driver.reset(&mut reset_output, &mut delay).ok();
    display_driver.init(&mut delay).ok();
    log::info!("Driver configured!");

    let mut touchpad = CST816S::new(i2c, cst816s_int1, cst816s_reset);
    touchpad.setup(&mut delay).unwrap();

    loop {
        // Int pin is not used.
        if let Some(event) = touchpad.read_one_touch_event(false) {
            log::info!("Touch event {:?}", event);
        }

        FreeRtos::delay_ms(500);

        let bat_mv = 4500; // Some fake battery voltage for now.
        let text = format!("VBAT: {:?}", bat_mv);

        let _ = display_driver.clear();
        let style = PrimitiveStyleBuilder::new()
            .stroke_width(2)
            .stroke_color(Rgb565::RED)
            .build();
        let _ = Circle::new(Point::new(100, 100), 20)
            .into_styled(style)
            .draw(&mut display_driver);
        let _ = Text::new(&text, Point::new(50, 50), MonoTextStyle::new(&FONT_6X10, Rgb565::RED))
            .draw(&mut display_driver);
        let _ = display_driver.flush();
    }
}