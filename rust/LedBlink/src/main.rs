use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use std::error::Error;
use std::thread;
use std::time::Duration;

const GPIO_LED:u8 = 17;



fn main()-> Result<(), Box<dyn Error>>  {
    println!("Blinking an LED on a {}", DeviceInfo::new()?.model());
    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    for _ in 1..10 {
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
    }
    println!("end");
    Ok(())
}
