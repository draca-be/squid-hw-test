use esp_idf_hal::gpio::{AnyIOPin, TouchPin};
use esp_idf_hal::prelude::Peripherals;
use crate::buttons::ButtonsConfiguration;
use crate::touch::TouchConfiguration;
use crate::leds::LedsConfiguration;

pub struct Hardware {
    pub leds: LedsConfiguration,
    pub touch: TouchConfiguration,
    pub buttons: ButtonsConfiguration,
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
        }
    }
}
