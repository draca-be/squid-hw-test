use embassy_time::{Duration, Timer};

pub fn init() {
    unsafe {
        // Disable IDLE task WatchDogTask on this CPU.
        esp_idf_sys::esp_task_wdt_delete(esp_idf_sys::xTaskGetIdleTaskHandleForCPU(esp_idf_hal::cpu::core() as u32));

        // Enable WatchDogTask on the main (=this) task.
        esp_idf_sys::esp_task_wdt_add(esp_idf_sys::xTaskGetCurrentTaskHandle());
    }
}

/// A very simple watchdog implementation
#[embassy_executor::task]
pub async fn start() {
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
