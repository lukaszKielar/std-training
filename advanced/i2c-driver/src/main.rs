use anyhow::Result;
use embedded_hal::delay::DelayNs;
use esp_idf_svc::hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};

use i2c_driver::icm42670p::{DeviceAddr, ICM42670P};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;

    let mut sensor = ICM42670P::new(i2c, DeviceAddr::AD0)?;

    println!("Sensor init");
    let device_id = sensor.read_device_id_register()?;

    println!("Hello, world, I am sensor {:#02x}", device_id);

    loop {
        FreeRtos.delay_ms(500u32);
    }
}
