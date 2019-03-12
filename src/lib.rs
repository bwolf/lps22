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

use cast::{i16, u16};
use core::mem;
use generic_array::typenum::consts::*;
use generic_array::{ArrayLength, GenericArray};
use hal::blocking::i2c::{Write, WriteRead};

/// I2c address depending on connection of SA0 PAD
#[allow(dead_code)] // TODO remove later
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum I2cAddress {
    /// TODO
    SA0_PAD_VOLTAGE = 0b1011101,
    /// TODO
    SA0_PAD_GROUND = 0b1011100,
}

#[allow(dead_code)]
impl I2cAddress {
    fn addr(&self) -> u8 {
        *self as u8
    }
}

/// Readable register address
trait ReadRegister {
    fn addr(&self) -> u8;
}

/// Readable and writable register address
trait WriteRegister {
    fn addr(&self) -> u8;
}

/// LPS22 Read/Write registers
#[allow(dead_code)] // TODO remove later
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum Lps22ReadWriteRegister {
    /// Interrupt register
    INTERRUPT_CFG = 0x0B,
    /// Pressure threshold register
    THS_P_L = 0x0C,
    /// Pressure threshold register
    THS_P_H = 0x0D,
    /// Interface control register
    IF_CTRL = 0x0E,
    /// Control register 1
    CTRL_REG1 = 0x10,
    /// Control register 2
    CTRL_REG2 = 0x11,
    /// Control register 3
    CTRL_REG3 = 0x12,
    /// FIFO configuration register
    FIFO_CTRL = 0x13,
    /// FIXME what's this? the data-sheet doesn't contain a doc. in the register table???
    FIFO_WTM = 0x14,
    /// Pressure offset register
    RPDS_L = 0x18,
    /// Pressure offset register
    RPDS_H = 0x19,
}

#[allow(dead_code)] // TODO remove
impl WriteRegister for Lps22ReadWriteRegister {
    fn addr(&self) -> u8 {
        *self as u8
    }
}

#[allow(dead_code)] // TODO remove
impl ReadRegister for Lps22ReadWriteRegister {
    fn addr(&self) -> u8 {
        *self as u8
    }
}

/// LPS22 Read only registers
#[allow(dead_code)] // TODO remove later
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum Lps22ReadRegister {
    /// Who am I
    WHO_AM_I = 0x0F,
    /// Reference pressure register
    REF_P_L = 0x15,
    /// Reference pressure register
    REF_P_H = 0x16,
    /// Interrupt register
    INT_SOURCE = 0x24,
    /// FIFO status register
    FIFO_STATUS1 = 0x25,
    /// FIFO status register
    FIFO_STATUS2 = 0x26,
    /// Status register
    STATUS = 0x27,
    /// Pressure output register
    PRESSURE_OUT_XL = 0x28,
    /// Pressure output register
    PRESSURE_OUT_L = 0x29,
    /// Pressure output register
    PRESSURE_OUT_H = 0x2A,
    /// Temperature output register
    TEMP_OUT_L = 0x2B,
    /// Temperature output register
    TEMP_OUT_H = 0x2C,
    /// FIFO pressure output register
    FIFO_DATA_OUT_PRESS_XL = 0x78,
    /// FIFO pressure output register
    FIFO_DATA_OUT_PRESS_L = 0x79,
    /// FIFO pressure output register
    FIFO_DATA_OUT_PRESS_H = 0x7A,
    /// FIFO temperature output register
    FIFO_DATA_OUT_TEMP_L = 0x7B,
    /// FIFO temperature output register
    FIFO_DATA_OUT_TEMP_H = 0x7C,
}

#[allow(dead_code)] // TODO remove
impl ReadRegister for Lps22ReadRegister {
    fn addr(&self) -> u8 {
        *self as u8
    }
}

/// LPS22 driver
#[allow(dead_code)]
pub struct Lps22<I2C> {
    address: I2cAddress,
    i2c: I2C,
}

impl<I2C, E> Lps22<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Create a new driver from a I2C peripheral.
    pub fn new(address: I2cAddress, i2c: I2C) -> Result<Self, E> {
        Ok(Lps22 { address, i2c })
    }

    #[allow(dead_code)] // TODO remove
    fn read_register<N, R>(&mut self, reg: R) -> Result<GenericArray<u8, N>, E>
    where
        N: ArrayLength<u8>,
        R: ReadRegister,
    {
        let mut buffer: GenericArray<u8, N> = unsafe { mem::uninitialized() };
        {
            let buffer: &mut [u8] = &mut buffer;
            self.i2c
                .write_read(self.address.addr(), &[reg.addr()], buffer)?;
        }
        Ok(buffer)
    }

    #[allow(dead_code)] // TODO remove
    fn write_register<R>(&mut self, reg: R, byte: u8) -> Result<(), E>
    where
        R: WriteRegister,
    {
        self.i2c.write(self.address.addr(), &[reg.addr(), byte])
    }

    #[allow(dead_code)] // TODO remove
    #[allow(missing_docs)] // TODO remove
    pub fn read(&mut self) -> Result<(), E> {
        self.write_register(Lps22ReadWriteRegister::CTRL_REG2, 0b00010000 | 1)?;

        let t: GenericArray<u8, U2> = self.read_register(Lps22ReadRegister::TEMP_OUT_L)?;
        let t: i16 = i16(t[0]) + (i16(t[1]) << 8);
        // TODO t/100 == degrees Celcius

        let p: GenericArray<u8, U3> = self.read_register(Lps22ReadRegister::PRESSURE_OUT_XL)?;
        // TODO data-sheet says signed decimal, why? The value cannot be negative!
        let p: u16 = u16(p[0]) + (u16(p[1]) << 8) + (u16(p[2]) << 16);
        // TODO p/4096 == hPa

        Ok(()) // TODO result structure
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
