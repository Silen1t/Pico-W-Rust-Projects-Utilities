#![no_std]
use gpio::{ Level, Output, Input };
use embassy_rp::gpio::{ self, AnyPin, Pull };
use embassy_time::{ Duration, Instant, Timer };
use ::{ defmt_rtt as _, panic_probe as _ };

pub struct UltraMeasure {
    echo: Input<'static>,
    trigger: Output<'static>,
}

impl UltraMeasure {
    pub fn new(echo_pin: AnyPin, trigger_pin: AnyPin) -> Self {
        Self {
            trigger: Output::new(trigger_pin, Level::Low),
            echo: Input::new(echo_pin, Pull::None),
        }
    }

    pub async fn measure_distance(&mut self) -> Result<f32, ()> {
        // Send trigger pulse
        self.trigger.set_high();
        Timer::after(Duration::from_micros(10)).await;
        self.trigger.set_low();

        // Wait for echo to go high (timeout: 25ms ≈ 400cm max)
        let start = Instant::now();
        while self.echo.is_low() {
            if Instant::now() - start > Duration::from_millis(25) {
                return Err(());
            }
        }

        // Measure echo pulse duration (timeout: 25ms)
        let start = Instant::now();
        while self.echo.is_high() {
            if Instant::now() - start > Duration::from_millis(25) {
                return Err(());
            }
        }
        let duration = Instant::now() - start;

        // Calculate distance
        let distance = (duration.as_micros() as f32) / 58.0;
        Ok(distance)
    }
}
