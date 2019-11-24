use distance;

use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello");

    let timeout = Duration::from_millis(100);

    let mut sensor = distance::DistanceSensor::new(1, 2, timeout)?;

    let cm = sensor.get_distance()?;

    println!("{}", cm);

    Ok(())
}
