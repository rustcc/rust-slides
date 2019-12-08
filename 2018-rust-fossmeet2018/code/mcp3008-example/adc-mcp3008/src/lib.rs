//! A platform agnostic driver to interface with the MCP3008 ADC.
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1
//!

//#![deny(missing_docs)]
//#![deny(warnings)]
#![feature(unsize)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::spi::{Transfer};
use hal::spi::{Mode, Phase, Polarity};
use hal::digital::OutputPin;

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

/// MCP3008 driver
pub struct Mcp3008<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS, E> Mcp3008<SPI, CS>
where
    SPI: Transfer<u8, Error = E>,
    CS: OutputPin,
{
    /// Creates a new driver from a SPI peripheral and a NCS pin
    pub fn new(spi: SPI, cs: CS) -> Result<Self, E> {
        let mcp3008 = Mcp3008 { spi, cs };

        Ok(mcp3008)
    }

    pub fn read_channel(&mut self, ch: Channel) -> Result<u16, E> {
        self.cs.set_low();

        let mut buffer = [0u8; 3];
        buffer[0] = 1;
        buffer[1] = ((1 << 3) | (ch as u8)) << 4;

        self.spi.transfer(&mut buffer)?;

        self.cs.set_high();

        let r = (((buffer[1] as u16) << 8) | (buffer[2] as u16)) & 0x3ff;
        Ok(r)
    }

}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Channel {
    CH0,
    CH1,
    CH2,
    CH3,
    CH4,
    CH5,
    CH6,
    CH7,
}
