use embedded_hal::spi::MODE_3;
use esp_idf_hal::gpio::{AnyIOPin, TouchPin};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::units::FromValueType;

use crate::buttons::ButtonsConfiguration;
use crate::display::DisplayConfiguration;
use crate::touch::TouchConfiguration;
use crate::leds::LedsConfiguration;

pub struct Hardware {
    pub leds: LedsConfiguration,
    pub touch: TouchConfiguration,
    pub buttons: ButtonsConfiguration,
    pub display: DisplayConfiguration,
}

impl Hardware {
    pub fn new() -> Hardware {
        let peripherals = Peripherals::take()
            .expect("Could not get hardware lock");


        Hardware {
            leds: LedsConfiguration {
                neopixels: AnyIOPin::from(peripherals.pins.gpio2),
            },
            touch: TouchConfiguration {
                touch_0: peripherals.pins.gpio27.touch_channel(),
                touch_1: peripherals.pins.gpio14.touch_channel(),
                touch_2: peripherals.pins.gpio13.touch_channel(),
            },
            buttons: ButtonsConfiguration {
                button_boot: AnyIOPin::from(peripherals.pins.gpio0),
            },
            display: DisplayConfiguration {
                spi: peripherals.spi2,
                sclk: AnyIOPin::from(peripherals.pins.gpio18), //sclk spi
                sdo: AnyIOPin::from(peripherals.pins.gpio23), //mosi spi
                sdi: AnyIOPin::from(peripherals.pins.gpio19),//miso spi
                cs: AnyIOPin::from(peripherals.pins.gpio5), //cs tft
                dc: AnyIOPin::from(peripherals.pins.gpio33), // tft
                rst: AnyIOPin::from(peripherals.pins.gpio32), // tft

                baudrate: 40.MHz().into(),
                mode: MODE_3,
            },
        }
    }
}
