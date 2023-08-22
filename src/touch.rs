use std::sync::atomic::{AtomicBool, Ordering};
use embassy_time::{Duration, Timer};
use esp_idf_sys::*;

pub struct TouchConfiguration {
    pub touch_0: touch_pad_t,
    pub touch_1: touch_pad_t,
    pub touch_2: touch_pad_t,
}

pub static TOUCH_0_ACTIVE: AtomicBool = AtomicBool::new(false);
pub static TOUCH_1_ACTIVE: AtomicBool = AtomicBool::new(false);
pub static TOUCH_2_ACTIVE: AtomicBool = AtomicBool::new(false);

pub fn init() {
    esp!(unsafe { touch_pad_init() }).expect("Could not init touch pad");
}

fn configure_touch_pad(pad: touch_pad_t) {
    esp!(unsafe { touch_pad_config(pad, 0)}).expect("Could not configure touch pad");
}

fn read_touch_pad(i: u8, pad: touch_pad_t, event: &AtomicBool) {
    let mut value: u16 = 0;

    esp!(unsafe { touch_pad_read(pad, &mut value) }).expect("Could not read touch 0");

    if value < 200 {
        if !event.load(Ordering::Relaxed) {
            println!("Touch {i} pressed");
            event.store(true, Ordering::Relaxed);
        }
    } else {
        if event.load(Ordering::Relaxed) {
            println!("Touch {i} released");
            event.store(false, Ordering::Relaxed);
        }
    }
}

#[embassy_executor::task]
pub async fn start(configuration: TouchConfiguration) {
    configure_touch_pad(configuration.touch_0);
    configure_touch_pad(configuration.touch_1);
    configure_touch_pad(configuration.touch_2);

    // This is highly inefficient, but that's not what we're aiming for here
    loop {
        read_touch_pad(0, configuration.touch_0, &TOUCH_0_ACTIVE);
        read_touch_pad(1, configuration.touch_1, &TOUCH_1_ACTIVE);
        read_touch_pad(2, configuration.touch_2, &TOUCH_2_ACTIVE);

        Timer::after(Duration::from_millis(100)).await;
    }
}
