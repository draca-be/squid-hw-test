use std::sync::atomic::Ordering;
use embassy_time::{Duration, Timer};
use esp_idf_hal::gpio::{AnyIOPin, Pin};
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds_trait::SmartLedsWrite;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;
use crate::buttons::BUTTON_BOOT_PRESSED;
use crate::touch::{TOUCH_0_ACTIVE, TOUCH_1_ACTIVE, TOUCH_2_ACTIVE};

pub struct LedsConfiguration {
    pub neopixels: AnyIOPin,
}

pub fn init() {}

#[embassy_executor::task]
pub async fn start(configuration: LedsConfiguration) {
    let mut ws2812 = Ws2812Esp32Rmt::new(
        0,
        configuration.neopixels.pin() as u32,
    ).unwrap();

    let mut subscriber = BUTTON_BOOT_PRESSED.subscriber().unwrap();

    println!("Start NeoPixel rainbow!");

    let hues_red: [u8; 5] = [0, 0, 0, 0, 0];
    let hues_yellow: [u8; 5] = [30, 30, 30, 30, 30];
    let hues_blue: [u8; 5] = [150, 150, 150, 150, 150];
    let mut hues_rainbow: [u8; 5] = [0, 50, 100, 150, 200];
    let mut val: u8 = 10;

    let mut counter: u8 = 0;

    loop {
        let mut hues = &hues_rainbow;

        if subscriber.try_next_message_pure().is_some() {
            val = val.wrapping_add(32);
        }

        let touch_0 = TOUCH_0_ACTIVE.load(Ordering::Relaxed);
        let touch_1 = TOUCH_1_ACTIVE.load(Ordering::Relaxed);
        let touch_2 = TOUCH_2_ACTIVE.load(Ordering::Relaxed);

        if touch_0 {
            hues = &hues_red;
        }

        if touch_1 {
            if ((touch_0 && touch_2) && (85 < counter && counter < 170))
                || !(touch_0 || touch_2)
                || (counter >= 127) {
                hues = &hues_yellow
            }
        }

        if touch_2 {
            if ((touch_0 && touch_1) && counter >= 170)
                || !(touch_0 || touch_1)
                || (!touch_0 && touch_1 && counter < 127)
                || (touch_0 && !touch_1 && counter >= 127)
            {
                hues = &hues_blue;
            }
        }

        let pixels = hues.map(|x| hsv2rgb(Hsv {
            hue: x,
            sat: 255,
            val: val,
        }));

        ws2812.write(pixels.into_iter()).unwrap();

        Timer::after(Duration::from_millis(50)).await;

        for hue in &mut hues_rainbow {
            *hue = hue.wrapping_add(5);
        }

        counter = counter.wrapping_add(20);
    }
}
