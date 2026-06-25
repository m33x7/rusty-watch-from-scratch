use cst816s::command::{IrqCtl, MotionMask, TouchEvent};
use cst816s::Cst816s;
use esp_idf_hal::gpio::{Gpio5, Gpio13, PinDriver, Pull};
use esp_idf_hal::task::block_on;
use esp_idf_hal::{delay::Delay, i2c};
use esp_idf_hal::delay::{FreeRtos};
use esp_idf_svc::sys;

use std::sync::Arc;
use crate::state;

pub struct AnotherTouchTaskData<'i2c, 'a> {
    pub i2c: &'a mut i2c::I2cDriver<'i2c>
}

pub struct TouchTaskData<'i2c, 'pins> {
    pub delay: Delay,
    pub i2c: i2c::I2cDriver<'i2c>,
    pub int1: Gpio5<'pins>,
    pub reset: Gpio13<'pins>,
    pub state: Arc<state::State>
}

pub fn setup_touch(touch: &mut Cst816s<&mut i2c::I2cDriver<'_>, Delay>,) -> anyhow::Result<()> {
    let mut irq_ctl = IrqCtl(0);
    irq_ctl.set_en_test(false);
    irq_ctl.set_en_touch(true);
    irq_ctl.set_en_change(true);
    irq_ctl.set_en_motion(true);
    irq_ctl.set_en_once_wlp(true);
    touch.write_irq_ctl(irq_ctl)?;

    let mut motion_mask = MotionMask(0);
    motion_mask.set_en_double_click(true);
    motion_mask.set_en_continuous_left_right(true);
    motion_mask.set_en_continuous_up_down(true);
    touch.write_motion_mask(motion_mask)?;

    touch.write_lp_scan_idac(1)?;
    touch.write_lp_scan_freq(7)?;
    touch.write_lp_scan_win(3)?;
    touch.write_lp_scan_th(48)?;
    touch.write_motion_s1_angle(0)?;
    touch.write_long_press_time(10)?;
    touch.write_auto_reset(5)?;

    Ok(())
}

pub fn touch_task(mut data: TouchTaskData) -> anyhow::Result<()> {
    let mut int1 = PinDriver::input(data.int1, Pull::Up)?;
    let mut reset_output = PinDriver::output(data.reset)?;

    let mut touch = Cst816s::new(&mut data.i2c, data.delay);

    touch.reset(&mut reset_output, &mut data.delay)?;

    setup_touch(&mut touch)?;

    touch.dump_register();

    unsafe {
        sys::gpio_wakeup_enable(
            int1.pin() as i32,
            sys::gpio_int_type_t_GPIO_INTR_LOW_LEVEL, // rising-edge-equivalent for sleep wakeup
        );

        // Tell the sleep subsystem GPIO wakeup is enabled as a source
        sys::esp_sleep_enable_gpio_wakeup();
    }

    loop {
        let wait_interrupt = block_on(int1.wait_for_rising_edge());

        if let Err(err) = wait_interrupt {
            log::error!("waiting on interupt error: {}", err);
        } else {
            let event = touch.read_events();
            log::info!("Touch event : {:?} ", event);
            data.state.register_touch();
        }
    }
}
