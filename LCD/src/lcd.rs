use defmt::*;
use embassy_rp::{
    bind_interrupts,
    i2c::{ self, Async, Config, I2c },
    peripherals::I2C0,
    peripherals::*,
};
use embassy_time::Delay;
use embedded_hal_1::delay::DelayNs;
use embassy_rp::gpio::{Input, Output, AnyPin, Level, Pull};
use hd44780_driver::{ bus::I2CBus, Cursor, CursorBlink, Display, HD44780 };
use ::{ defmt_rtt as _, panic_probe as _ };

#[derive(Debug, Clone, Copy)]
pub enum LcdError {
    ClearError,
    ResetError,
    DisplayError,
    WriteError,
    CursorError,
}

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

pub struct Lcd {
    driver: HD44780<I2CBus<I2c<'static, I2C0, Async>>>,
    delay: Delay,
}

impl Lcd {
    pub async fn new(i2c0: I2C0, scl: PIN_21, sda: PIN_20) -> Self {
        let config = Config::default();

        let i2c = I2c::new_async(
            i2c0,
            scl, // SCL - Check datasheet
            sda, // SDA - Check datasheet
            Irqs,
            config
        );

        // Create a mutable instance of Delay
        let delay: &mut Delay = &mut Delay;
        delay.delay_ms(100 as u32);
        let mut lcd = HD44780::new_i2c(i2c, 0x27, delay).unwrap();
        // self.initialize_lcd(&lcd, delay);
        match Self::initialize_lcd(&mut lcd, delay) {
            Ok(_) => {
                info!("Lcd is Okey");
            }
            Err(_) => {}
        }
        Self {
            driver: lcd,
            delay: Delay,
        }
    }

    fn initialize_lcd(
        lcd: &mut HD44780<I2CBus<I2c<'_, I2C0, Async>>>,
        delay: &mut Delay
    ) -> Result<(), LcdError> {
        lcd.clear(delay).map_err(|_e| LcdError::ClearError)?;
        lcd.reset(delay).map_err(|_e| LcdError::ResetError)?;
        lcd.set_display(Display::On, delay).map_err(|_e| LcdError::DisplayError)?;
        lcd.set_cursor_visibility(Cursor::Invisible, delay).map_err(|_e| LcdError::CursorError)?;
        lcd.set_cursor_blink(CursorBlink::Off, delay).map_err(|_e| LcdError::CursorError)?;
        Ok(())
    }

    pub async fn display_text(
        &mut self,
        text: &str,
        position: u8,
        clear_display: bool
    ) -> Result<(), LcdError> {
        self.delay.delay_ms(100);
        if clear_display {
            self.clear_display().await?;
            self.reset_display().await?;
        }
        self.set_cursor_pos(position).await?;
        self.driver.write_str(text, &mut self.delay).map_err(|_e| LcdError::WriteError)?;
        Ok(())
    }

    pub async fn display_byte(
        &mut self,
        byte: u8,
        position: u8,
        clear_display: bool
    ) -> Result<(), LcdError> {
        self.delay.delay_ms(100);
        if clear_display {
            self.clear_display().await?;
            self.reset_display().await?;
        }
        self.set_cursor_pos(position).await?;
        self.driver.write_byte(byte, &mut self.delay).map_err(|_e| LcdError::WriteError)?;
        Ok(())
    }

    pub async fn display_bytes(
        &mut self,
        bytes: &[u8],
        position: u8,
        clear_display: bool
    ) -> Result<(), LcdError> {
        self.delay.delay_ms(100);
        if clear_display {
            self.clear_display().await?;
            self.reset_display().await?;
        }
        self.set_cursor_pos(position).await?;
        self.driver.write_bytes(bytes, &mut self.delay).map_err(|_e| LcdError::WriteError)?;
        Ok(())
    }

    pub async fn display_char(
        &mut self,
        char: char,
        position: u8,
        clear_display: bool
    ) -> Result<(), LcdError> {
        self.delay.delay_ms(100);
        if clear_display {
            self.clear_display().await?;
            self.reset_display().await?;
        }
        self.set_cursor_pos(position).await?;
        self.driver.write_char(char, &mut self.delay).map_err(|_e| LcdError::WriteError)?;
        Ok(())
    }

    pub async fn clear_display(&mut self) -> Result<(), LcdError> {
        self.driver.clear(&mut self.delay).map_err(|_e| LcdError::ClearError)?;
        Ok(())
    }

    pub async fn reset_display(&mut self) -> Result<(), LcdError> {
        self.delay.delay_ms(100);
        self.driver.reset(&mut self.delay).map_err(|_e| LcdError::ResetError)?;
        Ok(())
    }

    pub async fn set_display_mode(&mut self, display_mode: Display) -> Result<(), LcdError> {
        self.delay.delay_ms(100);
        self.driver
            .set_display(display_mode, &mut self.delay)
            .map_err(|_e| LcdError::DisplayError)?;
        Ok(())
    }

    pub async fn set_cursor_visibility(
        &mut self,
        cursor_visibility: Cursor
    ) -> Result<(), LcdError> {
        self.delay.delay_ms(100);
        self.driver
            .set_cursor_visibility(cursor_visibility, &mut self.delay)
            .map_err(|_e| LcdError::CursorError)?;
        Ok(())
    }

    pub async fn set_cursor_blink(&mut self, cursor_blink: CursorBlink) -> Result<(), LcdError> {
        self.delay.delay_ms(100);
        self.driver
            .set_cursor_blink(cursor_blink, &mut self.delay)
            .map_err(|_e| LcdError::CursorError)?;
        Ok(())
    }

    pub async fn set_cursor_pos(&mut self, position: u8) -> Result<(), LcdError> {
        self.delay.delay_ms(100);
        self.driver.set_cursor_pos(position, &mut self.delay).map_err(|_e| LcdError::CursorError)?;
        Ok(())
    }
}
