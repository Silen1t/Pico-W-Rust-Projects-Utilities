#![no_std]

use embassy_rp::gpio::{AnyPin, Input, Pin, Pull};
use ::{ defmt_rtt as _, panic_probe as _ };

/// Tactile Button Handler with State Tracking
pub struct TactButton<'a> {
    pin: Input<'a>,
    previous_state: bool,
    current_state: bool,
}

impl TactButton<'_> {
    pub fn new(button_pin: AnyPin) -> Self {
        let pin = Input::new(button_pin, Pull::Up);
        let current_state = pin.is_high();
        Self {
            pin,
            previous_state: current_state,
            current_state,
        }
    }

    /// Update button state (should be called in main loop)
    pub fn update(&mut self) {
        self.previous_state = self.current_state;
        self.current_state = self.pin.is_high();
    }

    /// Returns true on button press (falling edge)
    pub fn is_pressed(&self) -> bool {
        !self.current_state && self.previous_state
    }

    /// Returns true on button release (rising edge)
    pub fn is_released(&self) -> bool {
        self.current_state && !self.previous_state
    }

    /// Returns true while button is held down
    pub fn is_held(&self) -> bool {
        !self.current_state
    }

    /// Returns true when button is in normal state (not pressed)
    pub fn is_normal(&self) -> bool {
        self.current_state
    }
}