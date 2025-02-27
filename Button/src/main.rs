#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{self, AnyPin, Input, Pull};
use embassy_time::Timer;
use gpio::{ Level, Output };
use tactbutton::TactButton;
use ::{ defmt_rtt as _, panic_probe as _ };

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut button = TactButton::new(AnyPin::from(p.PIN_15));
    let mut led = Output::new(p.PIN_16, Level::Low);


    loop {
        button.update();

        if button.is_held() {
            led.set_high();
        }else {
            led.set_low();
        }
    }
}
