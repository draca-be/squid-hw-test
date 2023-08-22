use embassy_time::{Duration, Timer};
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds_trait::SmartLedsWrite;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;


pub fn init() {

}

#[embassy_executor::task]
pub async fn start() {
    let mut ws2812 = Ws2812Esp32Rmt::new(0, 2).unwrap();

    println!("Start NeoPixel rainbow!");

    let mut hues: [u8; 5] = [0, 50, 100,150, 200];

    loop {
        let pixels = hues.map(|x| hsv2rgb(Hsv {
            hue: x,
            sat: 255,
            val: 90,
        }));

        ws2812.write(pixels.into_iter()).unwrap();

        Timer::after(Duration::from_millis(50)).await;

        for hue in &mut hues {
            *hue = hue.wrapping_add(5);
        }
    }
}
