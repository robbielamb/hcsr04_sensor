use hcsr04_sensor;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sensor = hcsr04_sensor::DistanceSensor::new(1, 2)?;

    let cm = sensor.get_distance()?;

    println!("{}", cm);

    Ok(())
}
