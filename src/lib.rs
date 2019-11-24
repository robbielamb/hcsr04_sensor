use rppal::gpio;

use std::error;
use std::fmt;
use std::result;
use std::thread;
use std::time::Duration;
use std::time::Instant;

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

pub struct DistanceSensor {
    trigger_pin: gpio::OutputPin,
    echo_pin: gpio::InputPin,
    timeout: Duration,
}

impl DistanceSensor {
    pub fn new(trigger: u8, echo: u8, timeout: Duration) -> Result<DistanceSensor> {
        let gpio = gpio::Gpio::new()?;
        let mut trigger_pin = gpio.get(trigger)?.into_output();
        let echo_pin = gpio.get(echo)?.into_input();

        trigger_pin.set_low();

        Ok(DistanceSensor {
            trigger_pin,
            echo_pin,
            timeout,
        })
    }

    /// Return the distance in Centameters
    pub fn get_distance(&mut self) -> Result<f64> {
        // Trigger the sensor
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

        // Track how long the pin is high;
        let start_instant = Instant::now();
        while self.echo_pin.is_high() {
            if start_instant.elapsed() > self.timeout {
                return Err(Error::TimeoutError);
            }
        }
        let duration = start_instant.elapsed();

        Ok(duration.as_secs_f64() * 170.0 * 100.0)
    }
}
