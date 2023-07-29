#![feature(type_alias_impl_trait)]

mod watchdog;

use embassy_executor::{Spawner, Executor};
use static_cell::StaticCell;

#[embassy_executor::task]
async fn run(spawner: Spawner) {
    spawner.spawn(watchdog::start()).unwrap();
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

    // There is no embassy_executor::main macro implemented yet so we manually define an executor
    static EXECUTOR: StaticCell<Executor> = StaticCell::new();
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(run(spawner)).unwrap();
    });
}
