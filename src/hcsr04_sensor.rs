//! This package is for getting the distance from an HC-SR04 ultrasonic range sensor.
//! Ported from https://github.com/adafruit/Adafruit_CircuitPython_HCSR04/blob/d7084483b3756e367ff01a09d7268a963267431c/adafruit_hcsr04.py
use rppal::gpio;

use std::error;
use std::fmt;
use std::result;
use std::thread;
use std::time::Duration;
use std::time::Instant;

/// Errors from using the sensor
#[derive(Debug)]
pub enum Error {
    GpioError(gpio::Error),
    TimeoutError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::GpioError(ref err) => err.fmt(f),
            Error::TimeoutError => write!(f, "Timeout waiting for sensor"),
        }
    }
}

impl error::Error for Error {}

impl From<gpio::Error> for Error {
    fn from(err: gpio::Error) -> Error {
        Error::GpioError(err)
    }
}

type Result<T> = result::Result<T, Error>;

/// Provide access to an HC-SR04 ultrasonic range sensor.
pub struct DistanceSensor {
    trigger_pin: gpio::OutputPin,
    echo_pin: gpio::InputPin,
    timeout: Duration,
}

impl DistanceSensor {
    /// Construct a new distance sensor running on the given GPIO pins.
    ///
    /// The device defaults to a 100ms timeout.
    pub fn new(trigger_pin: u8, echo_pin: u8) -> Result<DistanceSensor> {
        DistanceSensor::new_with_timeout(trigger_pin, echo_pin, Duration::from_millis(100))
    }

    /// Construct a new distance sensor running on the given GPIO pins with a provided timeout.
    pub fn new_with_timeout(
        trigger_pin: u8,
        echo_pin: u8,
        timeout: Duration,
    ) -> Result<DistanceSensor> {
        let gpio = gpio::Gpio::new()?;
        let mut trigger_pin = gpio.get(trigger_pin)?.into_output();
        let echo_pin = gpio.get(echo_pin)?.into_input();

        trigger_pin.set_low();

        Ok(DistanceSensor {
            trigger_pin,
            echo_pin,
            timeout,
        })
    }

    /// Return the distance in Centameters
    pub fn get_distance(&mut self) -> Result<f64> {
        // Trigger the sensor by setting it high then low again.
        self.trigger_pin.set_high();
        thread::sleep(Duration::from_micros(10));
        self.trigger_pin.set_low();

        let start_wait = Instant::now();

        // Wait until we see the pin go high
        while self.echo_pin.is_low() {
            if start_wait.elapsed() > self.timeout {
                return Err(Error::TimeoutError);
            }
        }

        // Track how long the pin is high. The duration the pin
        // is high should be
        let start_instant = Instant::now();
        while self.echo_pin.is_high() {
            if start_instant.elapsed() > self.timeout {
                return Err(Error::TimeoutError);
            }
        }
        let duration = start_instant.elapsed();

        // Sound travels at ~343 meters per second at 293.15K
        Ok(duration.as_secs_f64() * (343.0 / 2.0) * 100.0)
    }
}
