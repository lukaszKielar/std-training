use embedded_hal::delay::DelayNs as _;
use embedded_hal_bus::{i2c::AtomicDevice, util::AtomicCell};
use esp_idf_svc::hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use icm42670::Icm42670;
use shtcx::{self, shtc3, PowerMode};

#[derive(Debug)]
enum Error {
    I2cDriverInit,
    Sthc3Start,
    Sthc3UnreadableID,
    Sthc3Measurement,
    Icm42670Start,
    Icm42670UnreadableID,
    Icm42670GyroNorm,
}

type Result<T> = ::core::result::Result<T, Error>;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c =
        I2cDriver::new(peripherals.i2c0, sda, scl, &config).map_err(|_| Error::I2cDriverInit)?;
    let i2c_cell = AtomicCell::new(i2c);

    let mut sthc3 = shtc3(AtomicDevice::new(&i2c_cell));
    let sthc3_id = sthc3
        .device_identifier()
        .map_err(|_| Error::Sthc3UnreadableID)?;
    println!("Device ID SHTC3: {sthc3_id} [{:#02X}]", sthc3_id);

    let mut icm42670 = Icm42670::new(AtomicDevice::new(&i2c_cell), icm42670::Address::Primary)
        .map_err(|_| Error::Icm42670Start)?;
    let icm42670_id = icm42670
        .device_id()
        .map_err(|_| Error::Icm42670UnreadableID)?;
    println!("Device ID ICM42670p: {icm42670_id} [{:#02X}]", icm42670_id);

    loop {
        let gyro_data = icm42670.gyro_norm().map_err(|_| Error::Icm42670GyroNorm)?;
        println!(
            "GYRO: X: {:.2} Y: {:.2} Z: {:.2}",
            gyro_data.x, gyro_data.y, gyro_data.z
        );

        sthc3
            .start_measurement(PowerMode::NormalMode)
            .map_err(|_| Error::Sthc3Start)?;
        FreeRtos.delay_ms(100u32);

        let sthc3_measurement = sthc3
            .get_measurement_result()
            .map_err(|_| Error::Sthc3Measurement)?;
        println!(
            "TEMP: {:.2} °C",
            sthc3_measurement.temperature.as_degrees_celsius(),
        );
        println!("HUM: {:.2} %", sthc3_measurement.humidity.as_percent());

        FreeRtos.delay_ms(500u32);
    }
}
