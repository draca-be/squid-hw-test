use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::*;
use esp_idf_hal::spi::*;

use display_interface_spi::SPIInterfaceNoCS;
use embassy_time::{Duration, Timer};

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

use embedded_hal::spi::Mode;
use esp_idf_hal::prelude::Hertz;

use mipidsi::{Builder, ColorOrder, Orientation};
use mipidsi::ColorInversion::Inverted;

pub struct DisplayConfiguration {
    pub spi: SPI2,
    pub sclk: AnyIOPin,
    pub sdo: AnyIOPin,
    pub sdi: AnyIOPin,
    pub cs: AnyIOPin,
    pub dc: AnyIOPin,
    pub rst: AnyIOPin,

    pub baudrate: Hertz,
    pub mode: Mode,
}

pub fn init() {}

#[embassy_executor::task]
pub async fn start(configuration: DisplayConfiguration) {
    let rst = PinDriver::output(configuration.rst).expect("Could not init RST Pin");
    let dc = PinDriver::output(configuration.dc).expect("Could not init DC Pin");
    // let cs = PinDriver::output(configuration.cs).expect("Could not init CS Pin");

    let mut delay = Ets;

    let config = config::Config::new()
        .baudrate(configuration.baudrate)
        .data_mode(configuration.mode)
        // Needed to go above 26.7 MHz
        .write_only(true);

    let device = SpiDeviceDriver::new_single(
        configuration.spi,
        configuration.sclk,
        configuration.sdo,
        Some(configuration.sdi),
        Some(configuration.cs),
        &SpiDriverConfig::new(),
        &config,
    ).expect("Could not configure device driver");

    let di = SPIInterfaceNoCS::new(device, dc);

    // create driver
    let mut display = Builder::st7789(di)
        .with_display_size(240, 240)
        // set default orientation
        .with_orientation(Orientation::Portrait(false))
        .with_color_order(ColorOrder::Rgb)
        .with_invert_colors(Inverted)
        // initialize
        .init(&mut delay, Some(rst))
        .expect("Could not configure display");

    // let raw_image_data = ImageRawLE::new(include_bytes!("../examples/assets/ferris.raw"), 86);
    // let ferris = Image::new(&raw_image_data, Point::new(0, 0));

    loop {
        display.clear(Rgb565::RED).unwrap();
        Timer::after(Duration::from_millis(500)).await;
        display.clear(Rgb565::GREEN).unwrap();
        Timer::after(Duration::from_millis(500)).await;
        display.clear(Rgb565::BLUE).unwrap();
        Timer::after(Duration::from_millis(500)).await;
    }
}
