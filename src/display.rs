use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::*;
use esp_idf_hal::spi::*;

use display_interface_spi::SPIInterfaceNoCS;
use embassy_time::{Duration, Timer};
use embedded_graphics::mono_font::ascii::{FONT_10X20};
use embedded_graphics::mono_font::MonoTextStyle;

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle, RoundedRectangle};
use embedded_graphics::text::{Alignment, Text};

use embedded_hal::spi::Mode;
use esp_idf_hal::prelude::Hertz;

use mipidsi::{Builder, ColorOrder, Orientation};
use mipidsi::ColorInversion::Inverted;
use crate::wifi;

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

    let driver_config = config::DriverConfig::new()
        .dma(Dma::Auto(4096));

    let device = SpiDeviceDriver::new_single(
        configuration.spi,
        configuration.sclk,
        configuration.sdo,
        Some(configuration.sdi),
        Some(configuration.cs),
        &driver_config,
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

    let mut online = wifi::is_online();
    let mut previous = !online;
    loop {
        if previous != online {
            display.clear(Rgb565::BLACK).unwrap();

            let (color_rect, text, color_text) = if online {
                (Rgb565::GREEN, "Online!", Rgb565::BLACK)
            } else {
                (Rgb565::RED, "Offline...", Rgb565::WHITE)
            };

            let style = PrimitiveStyleBuilder::new()
                .stroke_width(1)
                .stroke_color(Rgb565::WHITE)
                .fill_color(color_rect)
                .build();

            RoundedRectangle::with_equal_corners(
                Rectangle::new(Point::new(10, 10), Size::new(220, 40)),
                Size::new(10, 10),
            )
                .into_styled(style)
                .draw(&mut display).unwrap();

            let style = MonoTextStyle::new(&FONT_10X20, color_text);

            Text::with_alignment(
                text,
                Point::new(120, 35),
                style,
                Alignment::Center,
            )
                .draw(&mut display).unwrap();

            if online {
                let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
                let ip = wifi::get_ip().await.to_string();

                Text::with_alignment(
                    &ip,
                    Point::new(10, 80),
                    style,
                    Alignment::Left,
                )
                    .draw(&mut display).unwrap();
            }

            previous = online;
        }

        Timer::after(Duration::from_secs(1)).await;
        online = wifi::is_online();
    }
}
