#![feature(type_alias_impl_trait)]

use esp_idf_sys as _;

use embassy_executor::{Spawner, Executor};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

/// A generic counter function
///
/// # Arguments
///
/// * `until` - The number to count to, counts to infinity if the value is 0
/// * `sleep` - The time to sleep between each count in ms
#[embassy_executor::task(pool_size = 2)]
async fn counter(id: u8, until: u32, sleep: u64) {
    let mut i = 0;

    while until == 0 || i <= until {
        println!("Loopcounter {id}: {i}");

        Timer::after(Duration::from_millis(sleep)).await;
        i += 1;
    }
}

/// A very simple watchdog implementation
#[embassy_executor::task]
async fn watchdog() {
    #[cfg(target_os = "espidf")]
    loop {
        unsafe {
            esp_idf_sys::esp_task_wdt_reset();
        }
        Timer::after(Duration::from_millis(2000)).await;
    }
    #[cfg(not(target_os = "espidf"))]
    println!("No watchdog available on this platform");
}

#[embassy_executor::task]
async fn run(spawner: Spawner) {
    spawner.spawn(watchdog()).unwrap();
    spawner.spawn(counter(0, 100, 1000)).unwrap();
    spawner.spawn(counter(1, 0, 200)).unwrap();
//    spawner.spawn(detect_button()).unwrap();
}

fn main() {
    // Patch the runtime for esp-idf-sys linking
    // https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Make sure critical-section gets linked in (required by StaticCell)
    esp_idf_hal::task::critical_section::link();

    // Make sure the time driver gets linked in
    esp_idf_svc::timer::embassy_time::queue::link();

    unsafe {
        // Disable IDLE task WatchDogTask on this CPU.
        esp_idf_sys::esp_task_wdt_delete(esp_idf_sys::xTaskGetIdleTaskHandleForCPU(esp_idf_hal::cpu::core() as u32));

        // Enable WatchDogTask on the main (=this) task.
        esp_idf_sys::esp_task_wdt_add(esp_idf_sys::xTaskGetCurrentTaskHandle());
    }

    println!("Hardware test Fri3D Camp squid badge (2022)");

    // There is no embassy_executor::main macro implemented yet so we manually define an executor
    static EXECUTOR: StaticCell<Executor> = StaticCell::new();
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(run(spawner)).unwrap();
    });
}
