use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use rppal::gpio::Level;

const GPIO_SENSOR:u8 = 4;

fn main()-> Result<(), Box<dyn Error>> {
    println!("sensor device={}", DeviceInfo::new()?.model());

    let sensor = Gpio::new()?.get(GPIO_SENSOR)?.into_input();

    for _ in 0..20 {
        thread::sleep(Duration::from_millis(1000));
        if sensor.read() == Level::High {
            println!("sensor High");
        } else {
            println!("sensor Low");
        }
    }

    Ok(())
}
