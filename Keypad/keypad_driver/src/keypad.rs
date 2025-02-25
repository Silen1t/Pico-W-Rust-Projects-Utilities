#![no_std]
use embassy_rp::gpio::{ Input, Output, AnyPin, Level, Pull };
use embassy_time::{ Duration, Timer };

pub struct Keypad<'a, const ROWS: usize, const COLS: usize> {
    rows: [Output<'a>; ROWS],
    cols: [Input<'a>; COLS],
}

impl<'a, const ROWS: usize, const COLS: usize> Keypad<'a, ROWS, COLS> {
    pub fn new(row_pins: [AnyPin; ROWS], col_pins: [AnyPin; COLS]) -> Self {
        let rows = row_pins.map(|pin| Output::new(pin, Level::High));
        let cols = col_pins.map(|pin| Input::new(pin, Pull::Up));

        Self { rows, cols }
    }

    pub async fn read(&mut self) -> Option<char> {
        // Iterate rows in reverse order
        for row_idx in (0..ROWS).rev() {
            let row = &mut self.rows[row_idx];

            row.set_low();
            Timer::after(Duration::from_millis(20)).await;

            for (col_idx, col) in self.cols.iter().enumerate() {
                if col.is_low() {
                    row.set_high();
                    return Some(Self::map_key(row_idx, col_idx));
                }
            }

            row.set_high();
        }
        None
    }

    #[inline]
    fn map_key(row: usize, col: usize) -> char {
        // Define the keypad layout
        const KEYMAP: [[char; 4]; 4] = [
            ['D', 'C', 'B', 'A'],
            ['#', '9', '6', '3'],
            ['0', '8', '5', '2'],
            ['*', '7', '4', '1'],
        ];
        KEYMAP[row][col]
    }
}
