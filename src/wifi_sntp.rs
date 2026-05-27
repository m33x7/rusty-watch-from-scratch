use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi},
    sntp::{EspSntp, SyncStatus},
};
use esp_idf_hal::peripherals::Peripherals;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc, TimeZone};
use chrono_tz::Europe::Berlin;

const WIFI: &str = env!("WIFI");
const WIFI_PWD: &str = env!("WIFI_PWD");

// TODO - sync every hour.
// TODO - set friedly device name. Now it shows as "espressif".
pub fn wifi_get_timestamp<'a>(mut wifi: BlockingWifi<EspWifi<'a>>) -> anyhow::Result<i64> {
    log::info!("WIFI: {WIFI}");
    log::info!("WIFI_PWD: {WIFI_PWD}");

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid:     WIFI.try_into().unwrap(),
        password: WIFI_PWD.try_into().unwrap(),
        ..Default::default()
    }))?;

    wifi.start()?;
    wifi.connect()?;

    log::info!("Connected ");
    wifi.wait_netif_up()?;         // DHCP done, IP assigned

    // --- NTP sync ---
    let sntp = EspSntp::new_default()?;

    while sntp.get_sync_status() != SyncStatus::Completed {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // --- read timestamp ---
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let epoch_us = now.as_secs() as i64 * 1_000_000
                 + now.subsec_micros() as i64;

    // wifi + sntp drop here, modem powers down automatically
    Ok(epoch_us)
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