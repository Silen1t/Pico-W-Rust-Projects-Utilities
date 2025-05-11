#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{ AnyPin, Level, Output };
use embassy_time::{ Duration, Timer };
use ::{ defmt_rtt as _, panic_probe as _ };
use lcd_driver::{ CursorMoveDirection, Lcd };
use keypad_driver::Keypad;

const TRY_AGAIN_TEXT: &str = "Try Again 'C'";

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    
    // Init LEDs
    let mut red_light = Output::new(p.PIN_14, Level::Low);
    let mut green_light = Output::new(p.PIN_15, Level::Low);

    // Init LCD
    let mut lcd = Lcd::new(p.I2C0, p.PIN_21, p.PIN_20).await;
    lcd.set_cursor_pos(0).await.unwrap();
    lcd.display_text("Enter Password:", true).await.unwrap();
    lcd.set_cursor_visibility(true).await.unwrap();

    // Init Keypad
    let mut keypad = Keypad::new(
        [
            AnyPin::from(p.PIN_2),
            AnyPin::from(p.PIN_3),
            AnyPin::from(p.PIN_4),
            AnyPin::from(p.PIN_5),
        ],
        [AnyPin::from(p.PIN_6), AnyPin::from(p.PIN_7), AnyPin::from(p.PIN_8), AnyPin::from(p.PIN_9)]
    );

    const PASSWORD: [Option<char>; 4] = [Some('7'), Some('1'), Some('6'), Some('9')];
    let mut user_buff: [Option<char>; 4] = [None; 4];
    let mut index_buff: usize = 0;
    lcd.set_cursor_pos(40).await.unwrap();

    loop {
        if let Some(key) = keypad.read().await {
            if key.is_numeric() {
                // Check if there's space in the buffer
                if index_buff < PASSWORD.len() {
                    // Store the key in the buffer
                    user_buff[index_buff] = Some(key);
                    // Display the character on the LCD
                    lcd.display_char(key, false).await.unwrap();
                    // Increment the buffer index
                    index_buff += 1;
                }
            } else if key.is_alphabetic() {
                // Eneter Button: check if the user password is true
                if key == 'A' {
                    if index_buff == PASSWORD.len() {
                        if PASSWORD == user_buff {
                            password_correct(&mut red_light, &mut green_light, &mut lcd).await;
                        } else {
                            password_incorrect(&mut red_light, &mut green_light, &mut lcd).await;
                        }
                    } else {
                        password_incorrect(&mut red_light, &mut green_light, &mut lcd).await;
                    }
                }
                // Try Again Button
                if key == 'C' {
                    try_again(&mut red_light, &mut green_light, &mut lcd).await;
                    user_buff = [None; 4];
                    index_buff = 0;
                }
                // Remove Last Number
                if key == 'D' {
                    if index_buff > 0 {
                        lcd.move_cursor_direction(CursorMoveDirection::Left).await.unwrap();
                        lcd.display_char(' ', false).await.unwrap();
                        lcd.move_cursor_direction(CursorMoveDirection::Left).await.unwrap();
                        index_buff -= 1;
                    }
                }
            }
        }
        Timer::after(Duration::from_micros(20)).await;
    }
}

async fn password_correct(
    red_light: &mut Output<'static>,
    green_light: &mut Output<'static>,
    lcd: &mut Lcd
) {
    red_light.set_low();
    green_light.set_high();
    lcd.set_cursor_pos(0).await.unwrap();
    lcd.display_text("Password Correct!", true).await.unwrap();
    lcd.set_cursor_pos(40).await.unwrap();
    lcd.display_text(TRY_AGAIN_TEXT, false).await.unwrap();
}

async fn password_incorrect(
    red_light: &mut Output<'static>,
    green_light: &mut Output<'static>,
    lcd: &mut Lcd
) {
    red_light.set_high();
    green_light.set_low();
    lcd.display_text("Please Try Again", true).await.unwrap();
    lcd.set_cursor_pos(40).await.unwrap();
    lcd.display_text(TRY_AGAIN_TEXT, false).await.unwrap();
}

async fn try_again(
    red_light: &mut Output<'static>,
    green_light: &mut Output<'static>,
    lcd: &mut Lcd
) {
    lcd.set_cursor_pos(0).await.unwrap();
    lcd.display_text("Enter Password:", true).await.unwrap();
    lcd.set_cursor_pos(40).await.unwrap();
    green_light.set_low();
    red_light.set_low();
}
