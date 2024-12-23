use embedded_hal::delay::DelayNs as _;
use esp_idf_svc::hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use shtcx::{self, shtc3, PowerMode};

#[derive(Debug)]
enum Error {
    I2cDriverInit,
    Sthc3Start,
    Sthc3UnreadableID,
    Sthc3Measurement,
}

type Result<T> = ::core::result::Result<T, Error>;

// Goals of this exercise:
// - Part2: Implement second sensor on same bus to solve an ownership problem

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c =
        I2cDriver::new(peripherals.i2c0, sda, scl, &config).map_err(|_| Error::I2cDriverInit)?;

    let mut sthc3 = shtc3(i2c);

    let sthc3_id = sthc3
        .device_identifier()
        .map_err(|_| Error::Sthc3UnreadableID)?;
    println!("Device ID SHTC3: {sthc3_id} [{:#X}]", sthc3_id);

    loop {
        sthc3
            .start_measurement(PowerMode::NormalMode)
            .map_err(|_| Error::Sthc3Start)?;

        let measurement = sthc3
            .get_measurement_result()
            .map_err(|_| Error::Sthc3Measurement)?;
        FreeRtos.delay_ms(100u32);

        println!(
            "TEMP: {} °C | HUM: {} %",
            measurement.temperature.as_degrees_celsius(),
            measurement.humidity.as_percent()
        );

        FreeRtos.delay_ms(500u32);
    }
}
