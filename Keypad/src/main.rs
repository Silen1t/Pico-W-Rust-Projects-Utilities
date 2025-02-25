#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{ gpio::AnyPin, Peripherals };
use embassy_time::{ Duration, Timer };
use keypad_driver::Keypad;
use lcd_driver::Lcd;

use ::{ defmt_rtt as _, panic_probe as _ };

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p: Peripherals = embassy_rp::init(Default::default());

    // Define LCD pin
    let mut lcd = Lcd::new(p.I2C0, p.PIN_21, p.PIN_20).await;
    lcd.display_text("Hello World!", 0, true).await.unwrap();

    let mut keypad = Keypad::new(
        [
            AnyPin::from(p.PIN_2),
            AnyPin::from(p.PIN_3),
            AnyPin::from(p.PIN_4),
            AnyPin::from(p.PIN_5),
        ],
        [AnyPin::from(p.PIN_6), AnyPin::from(p.PIN_7), AnyPin::from(p.PIN_8), AnyPin::from(p.PIN_9)]
    );

    loop {
        if let Some(key) = keypad.read().await {
            lcd.display_char(key, 0, true).await.unwrap();
        }

        Timer::after(Duration::from_millis(10)).await; // Reduced delay for faster polling
    }
}
