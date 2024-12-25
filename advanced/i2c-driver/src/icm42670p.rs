#![deny(unsafe_code)]

use embedded_hal::i2c;

/// ICM42670P device driver, represented by a struct with 2 fields.
/// Datasheet: https://invensense.tdk.com/wp-content/uploads/2021/07/DS-000451-ICM-42670-P-v1.0.pdf
#[derive(Debug)]
pub struct ICM42670P<I2C> {
    i2c: I2C,
    address: DeviceAddr,
}

/// Contains the possible variants of the devices addesses as binary numbers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceAddr {
    AD0 = 0b110_1000,
    AD1 = 0b110_1001,
}

impl<I2C, E> ICM42670P<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    /// Creates a new instance of the sensor, taking ownership of the i2c peripheral.
    pub fn new(i2c: I2C, address: DeviceAddr) -> Result<Self, E> {
        Ok(Self { i2c, address })
    }

    /// Returns the device's ID `0x67
    pub fn read_device_id_register(&mut self) -> Result<u8, E> {
        let mut buf = [0u8];

        self.i2c
            .write_read(self.address as u8, &[Register::WhoAmI.address()], &mut buf)
            .map(|_| Ok(u8::from_le_bytes(buf)))?
    }
}

/// This enum represents the device's registers
#[derive(Clone, Copy)]
pub enum Register {
    WhoAmI = 0x75,
}

impl Register {
    fn address(&self) -> u8 {
        *self as u8
    }
}
