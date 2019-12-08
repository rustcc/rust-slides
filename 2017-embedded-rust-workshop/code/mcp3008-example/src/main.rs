
//! Raspberry Pi MCP3008 demo
//!
//! # Connections
//!
//! IMPORTANT: Do *not* use PIN24 / BCM8 / CE0 as the NCS pin
//!
//! - PIN1 = 3V3 = VCC
//! - PIN19 = BCM10 = MOSI (SDA)
//! - PIN21 = BCM9 = MISO (AD0)
//! - PIN23 = BCM11 = SCLK (SCL)
//! - PIN22 = BCM25 = NCS
//! - PIN6 = GND = GND

extern crate linux_embedded_hal as hal;
extern crate adc_mcp3008;

use std::thread;
use std::time::Duration;

use adc_mcp3008::{Mcp3008, Channel};
use hal::spidev::{self, SpidevOptions};
use hal::{Pin, Spidev};
use hal::sysfs_gpio::Direction;

fn main() {
    let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(1_000_000)
        .mode(spidev::SPI_MODE_0)
        .build();
    spi.configure(&options).unwrap();

    let ncs = Pin::new(25);
    ncs.export().unwrap();
    while !ncs.is_exported() {}
    ncs.set_direction(Direction::Out).unwrap();
    ncs.set_value(1).unwrap();

    let mut mcp3008 = Mcp3008::new(spi, ncs).unwrap();

    loop {
        println!("{:?}", mcp3008.read_channel(Channel::CH7));
        thread::sleep(Duration::from_millis(1000));
    }
}
