//! Morse output for embedded platforms
//!
//! # Limitations
//!
//! Only supports 'a-zA-Z '
//!
//! # Example
//!
//! ```
//! let pin = …;
//! let delay = …;
//!
//! let mut morse = Morse::new_default(delay, pin, false);
//! morse.output_str("Hello World");
//! ```
#![no_std]

use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

/// 0 is dot, 1 is dash
#[derive(Debug, Clone, Copy)]
struct MorseChar {
    length: u8,
    pattern: u8,
}

const CHARS: [MorseChar; 26] = [
    // A
    MorseChar {
        length: 2,
        pattern: 0b10,
    },
    // B
    MorseChar {
        length: 4,
        pattern: 0b0001,
    },
    // C
    MorseChar {
        length: 4,
        pattern: 0b0101,
    },
    // D
    MorseChar {
        length: 3,
        pattern: 0b001,
    },
    // E
    MorseChar {
        length: 1,
        pattern: 0b0,
    },
    // F
    MorseChar {
        length: 4,
        pattern: 0b0100,
    },
    // G
    MorseChar {
        length: 3,
        pattern: 0b011,
    },
    // H
    MorseChar {
        length: 4,
        pattern: 0b0000,
    },
    // I
    MorseChar {
        length: 2,
        pattern: 0b00,
    },
    // J
    MorseChar {
        length: 4,
        pattern: 0b1110,
    },
    // K
    MorseChar {
        length: 3,
        pattern: 0b101,
    },
    // L
    MorseChar {
        length: 4,
        pattern: 0b0010,
    },
    // M
    MorseChar {
        length: 2,
        pattern: 0b11,
    },
    // N
    MorseChar {
        length: 2,
        pattern: 0b01,
    },
    // O
    MorseChar {
        length: 3,
        pattern: 0b111,
    },
    // P
    MorseChar {
        length: 4,
        pattern: 0b0110,
    },
    // Q
    MorseChar {
        length: 4,
        pattern: 0b1011,
    },
    // R
    MorseChar {
        length: 3,
        pattern: 0b010,
    },
    // S
    MorseChar {
        length: 3,
        pattern: 0b111,
    },
    // T
    MorseChar {
        length: 1,
        pattern: 0b1,
    },
    // U
    MorseChar {
        length: 3,
        pattern: 0b100,
    },
    // V
    MorseChar {
        length: 4,
        pattern: 0b1000,
    },
    // W
    MorseChar {
        length: 3,
        pattern: 0b110,
    },
    // X
    MorseChar {
        length: 4,
        pattern: 0b1001,
    },
    // Y
    MorseChar {
        length: 4,
        pattern: 0b1101,
    },
    // Z
    MorseChar {
        length: 4,
        pattern: 0b0011,
    },
];

pub struct Morse<DELAY, PIN> {
    dot_length: u16,
    dash_length: u16,
    space_length: u16,
    delay: DELAY,
    pin: PIN,
    invert: bool,
}

impl<ERR, DELAY: DelayMs<u16>, PIN: OutputPin<Error = ERR>> Morse<DELAY, PIN> {
    /// Create a new morse instance with a configurable dot_length in ms
    /// `invert` inverts the output signal, so that the output is set low, when it's active
    pub fn new(delay: DELAY, pin: PIN, invert: bool, dot_length: u16) -> Self {
        Self {
            dot_length,
            dash_length: dot_length * 3,
            space_length: dot_length * 3,
            delay,
            pin,
            invert,
        }
    }
    /// Create a new morse instance with a `dot_length` of 300 ms
    /// `invert` inverts the output signal, so that the output is set low, when it's active
    pub fn new_default(delay: DELAY, pin: PIN, invert: bool) -> Self {
        Self::new(delay, pin, invert, 300)
    }

    /// Output a string as a morse message
    ///
    /// Only supports 'a-zA-Z '
    pub fn output_str(&mut self, output: &str) -> Result<(), ERR> {
        for c in output.chars() {
            let c = c.to_ascii_uppercase();
            if c.is_ascii_uppercase() {
                let morse_char = CHARS[c as usize - 0x41];
                let mut pattern = morse_char.pattern;
                for _ in 0..morse_char.length {
                    if self.invert {
                        self.pin.set_low()?;
                    } else {
                        self.pin.set_high()?;
                    }
                    self.delay.delay_ms(if pattern & 0b1 == 1 {
                        self.dash_length
                    } else {
                        self.dot_length
                    });
                    if self.invert {
                        self.pin.set_high()?;
                    } else {
                        self.pin.set_low()?;
                    }
                    pattern = pattern >> 1;
                    self.delay.delay_ms(self.dot_length);
                }
                self.delay.delay_ms(self.space_length);
            } else if c == ' ' {
                self.delay.delay_ms(self.dot_length * 7);
            }
        }
        Ok(())
    }
}
