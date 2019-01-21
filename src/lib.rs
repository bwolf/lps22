//! A platform agnostic driver to interface the [ST][ST] LPS22
//! pressure sensor, written in Rust. Although the driver is verified
//! using a [`LPS22HH`][LPS22HH] sensor, it should be in theory
//! compatible to other `LPS22` sensors like the [`LPS22HB`][LPS22HB]
//! and if no, it should be easy to make it so.
//!
//! This driver is build using the [embedded-hal][embedded-hal] traits.
//!
//! [ST]: https://www.st.com/en/mems-and-sensors/pressure-sensors.html?querycriteria=productId=SC1316
//! [LPS22HH]: https://www.st.com/en/mems-and-sensors/lps22hh.html
//! [LPS22HB]: https://www.st.com/en/mems-and-sensors/lps22hb.html
//! [embedded-hal]: https://docs.rs/embedded-hal/
//!
//! ## Features
//!
//! TODO TBD

#![no_std]
#![deny(missing_docs)]

use hal::blocking::i2c::{Write, WriteRead};

/// LPS22 driver
#[allow(dead_code)]
pub struct Lps22<I2C> {
    i2c: I2C,
}

impl<I2C, E> Lps22<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Create a new driver from a I2C peripheral.
    pub fn new(i2c: I2C) -> Result<Self, E> {
        Ok(Lps22 { i2c })
    }
}

#[cfg(test)]
mod tests {
    // TODO
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
