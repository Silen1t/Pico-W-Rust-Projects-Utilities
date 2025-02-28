#![no_std]
#![no_main]

// use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::AnyPin;
use embassy_time::{ Duration, Timer };
use ::{ defmt_rtt as _, panic_probe as _ };
use lcd_driver::Lcd;
use ultra_measure::UltraMeasure;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Init LCD
    let mut lcd = Lcd::new(p.I2C0, p.PIN_21, p.PIN_20).await;

    // Init Sensor
    let mut sensor = UltraMeasure::new(AnyPin::from(p.PIN_16), AnyPin::from(p.PIN_15)) ;

    loop {
        // Calculate distance in centimeters
        match sensor.measure_distance().await {
            Ok(distance) => {
                lcd.display_text("Distance:", true).await.unwrap();
                lcd.display_int(distance as i32, false).await.unwrap();
                lcd.display_text("cm", false).await.unwrap();
            }
            Err(_) => todo!(),
        }

        // Wait before next measurement
        Timer::after(Duration::from_millis(500)).await;
    }
}

