#![no_std]
#![no_main]

mod lcd;
mod keypad;
use defmt::{ info, Format };
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{ AnyPin, Input, Level, Output, Pull },
    peripherals::*,
    Peripherals,
};
use embassy_time::{ Duration, Timer };
use keypad::Keypad;
use lcd::Lcd;
use ::{ defmt_rtt as _, panic_probe as _ };

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p: Peripherals = embassy_rp::init(Default::default());

    // Define LED pin
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

    // Define the correct password
    const PASSWORD: &[char] = &['1', '2', '3', '4']; // Example password: "1234"
    let mut input_buffer = [None; 4]; // Buffer to store user input
    let mut input_index = 0;

    loop {
        if let Some(key) = keypad.read().await {
            lcd.display_char(key, 0, true).await.unwrap();

            // Store the key in the input buffer
            if input_index < input_buffer.len() {
                input_buffer[input_index] = Some(key);
                input_index += 1;
            }

            // Check if the buffer is full
            if input_index == input_buffer.len() {
                // Compare the input buffer with the password
                let mut correct = true;
                for (i, &expected) in PASSWORD.iter().enumerate() {
                    if input_buffer[i] != Some(expected) {
                        correct = false;
                        break;
                    }
                }

                // If the password is correct, light up the LED
                if correct {
                    defmt::info!("Correct password! Lighting up the LED.");
                    lcd.display_text("Correct password!", 0, true).await.unwrap();
                } else {
                    defmt::info!("Incorrect password!");
                }

                // Reset the input buffer
                input_buffer = [None; 4];
                input_index = 0;
            }
        }

        Timer::after(Duration::from_millis(10)).await; // Reduced delay for faster polling
    }
}
