use std::net::Ipv4Addr;
use std::sync::atomic::{AtomicBool, Ordering};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};

pub struct WifiConfiguration {
    pub modem: Modem,
}

pub fn init() {}

static ONLINE: AtomicBool = AtomicBool::new(false);
static IP: Mutex<CriticalSectionRawMutex, Ipv4Addr> = Mutex::new(Ipv4Addr::new(127, 0, 0, 1));

pub fn is_online() -> bool {
    ONLINE.load(Ordering::Relaxed)
}

pub async fn get_ip() -> Ipv4Addr {
    *IP.lock().await
}

fn set_online(online: bool) {
    ONLINE.store(online, Ordering::Relaxed);
}

async fn set_ip(ip: Ipv4Addr) {
    *IP.lock().await = ip;
}
async fn connect(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    if wifi.is_started()? {
        set_online(false);
        println!("Wifi was started before, stopping.");

        wifi.stop().await?;
    }

    println!("Connecting wifi");

    let config = Configuration::Client(
        ClientConfiguration {
            ssid: env!("WIFI_SSID").into(),
            bssid: None,
            auth_method: AuthMethod::WPA2Personal,
            password: env!("WIFI_PASSWORD").into(),
            channel: None,
        }
    );

    wifi.set_configuration(&config)?;

    wifi.start().await?;
    println!("Wifi started");

    wifi.connect().await?;
    println!("Wifi connected");

    wifi.wait_netif_up().await?;
    println!("Wifi netif up");

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    println!("Wifi DHCP info: {:?}", ip_info);

    set_online(true);
    set_ip(ip_info.ip).await;

    wifi.wifi_wait(|| wifi.is_connected(), None).await?;
    println!("Wifi is down");

    set_online(false);

    Ok(())
}

#[embassy_executor::task]
pub async fn start(configuration: WifiConfiguration) {
    let sys_loop = EspSystemEventLoop::take().expect("Could not grab event loop");
    let timer_service = EspTaskTimerService::new().expect("Could not get timer");
    let nvs = EspDefaultNvsPartition::take().expect("Could not get NVS");

    let mut wifi = AsyncWifi::wrap(
        EspWifi::new(configuration.modem, sys_loop.clone(), Some(nvs)).expect("Could not get wifi access"),
        sys_loop,
        timer_service.clone(),
    ).expect("Could not create wifi driver");

    loop {
        match connect(&mut wifi).await {
            Ok(()) => (),
            Err(error) => {
                println!("Could not connect wifi: {error}");
            }
        }

        println!("Waiting 5 seconds to retry wifi");
        Timer::after(Duration::from_secs(5)).await;
    }
}
