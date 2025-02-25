#![no_std]
#![no_main]

use embassy_rp::Peripherals;
use lcd_driver::Lcd;
use ::{ defmt_rtt as _, panic_probe as _ };


#[embassy_executor::main]
async fn main(_task_spawner: embassy_executor::Spawner) {
    let p: Peripherals = embassy_rp::init(Default::default());

    // Define LCD pin
    let mut lcd = Lcd::new(p.I2C0, p.PIN_21, p.PIN_20).await;
    lcd.display_text("Hello World!", true).await.unwrap();

    loop {

    }
}

