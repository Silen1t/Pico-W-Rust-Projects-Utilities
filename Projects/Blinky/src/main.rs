#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::*;
use embassy_time::Timer;
use ::{ defmt_rtt as _, panic_probe as _ };

#[embassy_executor::main]
async fn main(_spawner: Spawner) {

    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_15, Level::High);
    
    loop {
        // led on!
        led.set_high();
        Timer::after_secs(1).await;

        // led off!
        led.set_low();
        Timer::after_secs(1).await;
    }
}