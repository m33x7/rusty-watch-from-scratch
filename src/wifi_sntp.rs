use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi},
    sntp::{EspSntp, SyncStatus},
};

const WIFI: &str = env!("WIFI");
const WIFI_PWD: &str = env!("WIFI_PWD");

// TODO - sync every hour.
// TODO - set friedly device name. Now it shows as "espressif".
pub fn sync_clocks<'a>(mut wifi: BlockingWifi<EspWifi<'a>>) -> anyhow::Result<()> {
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

    Ok(())
}