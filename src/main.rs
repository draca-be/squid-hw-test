#![feature(type_alias_impl_trait)]

mod leds;
mod watchdog;
mod hardware;
mod touch;
mod buttons;
mod display;

use embassy_executor::{Spawner, Executor};
use static_cell::StaticCell;
use crate::hardware::Hardware;

#[embassy_executor::task]
async fn run(spawner: Spawner) {
    let hardware = Hardware::new();

    spawner.spawn(watchdog::start()).unwrap();
    spawner.spawn(leds::start(hardware.leds)).unwrap();
    spawner.spawn(touch::start(hardware.touch)).unwrap();
    spawner.spawn(buttons::start(hardware.buttons)).unwrap();
    spawner.spawn(display::start(hardware.display)).unwrap();
}

fn main() {
    // Patch the runtime for esp-idf-sys linking
    // https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Make sure critical-section gets linked in (required by StaticCell)
    esp_idf_hal::task::critical_section::link();

    // Make sure the time driver gets linked in
    esp_idf_svc::timer::embassy_time::queue::link();

    println!("Hardware test Fri3D Camp squid badge (2022)");

    watchdog::init();
    leds::init();
    touch::init();
    buttons::init();
    display::init();

    // There is no embassy_executor::main macro implemented yet so we manually define an executor
    static EXECUTOR: StaticCell<Executor> = StaticCell::new();
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(run(spawner)).unwrap();
    });
}
