use embassy_time::{Duration, Timer};

pub fn init() {
    unsafe {
        // Enable WatchDogTask on the main (=this) task.
        esp_idf_sys::esp_task_wdt_add(esp_idf_sys::xTaskGetCurrentTaskHandle());
    }
}

/// A very simple watchdog implementation
#[embassy_executor::task]
pub async fn start() {
    loop {
        unsafe {
            esp_idf_sys::esp_task_wdt_reset();
        }
        Timer::after(Duration::from_millis(2000)).await;
    }
}
