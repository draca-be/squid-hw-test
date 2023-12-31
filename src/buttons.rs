use std::sync::atomic::{AtomicBool, Ordering};
use esp_idf_hal::gpio::{AnyIOPin, PinDriver, Pull};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::pubsub::PubSubChannel;

pub struct ButtonsConfiguration {
    pub button_boot: AnyIOPin,
}

pub fn init() {}

pub static BUTTON_BOOT_ACTIVE: AtomicBool = AtomicBool::new(false);
pub static BUTTON_BOOT_PRESSED: PubSubChannel<CriticalSectionRawMutex, bool, 4, 4, 1> = PubSubChannel::new();

#[embassy_executor::task]
pub async fn start(configuration: ButtonsConfiguration) {
    let mut button_boot = PinDriver::input(configuration.button_boot)
        .expect("Could not initialize boot button");

    button_boot.set_pull(Pull::Down).expect("Could not configure boot button");

    let publisher = BUTTON_BOOT_PRESSED.publisher().unwrap();

    loop {
        match button_boot.wait_for_falling_edge().await {
            Ok(()) => {
                if !BUTTON_BOOT_ACTIVE.load(Ordering::Relaxed) {
                    println!("Boot button pressed");
                    BUTTON_BOOT_ACTIVE.store(true, Ordering::Relaxed);
                    publisher.publish_immediate(true);
                }

                match button_boot.wait_for_rising_edge().await {
                    Ok(()) => {
                        if BUTTON_BOOT_ACTIVE.load(Ordering::Relaxed) {
                            println!("Boot button released");
                            BUTTON_BOOT_ACTIVE.store(false, Ordering::Relaxed);
                        }
                    }
                    Err(error) => {
                        println!("Error detecting boot button release: {error}");
                    }
                }
            }
            Err(error) => {
                println!("Error detecting boot button press: {error}");
            }
        }
    }
}
