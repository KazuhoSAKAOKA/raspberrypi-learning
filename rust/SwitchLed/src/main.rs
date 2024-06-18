use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, Level};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const GPIO_LED:u8 = 18;
const GPIO_SWITCH:u8 = 17;


fn main()->Result<(), Box<dyn Error>> {

    println!("switch and led");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");


    let mut led = Gpio::new()?.get(GPIO_LED)?.into_output();
    led.set_high();

    let switch = Gpio::new()?.get(GPIO_SWITCH)?.into_input();
    let mut before_level = switch.read();

    while running.load(Ordering::SeqCst){
        thread::sleep(Duration::from_millis(100));

        let current_level = switch.read();
        if current_level != before_level {
            if current_level == Level::High {
                println!("input high");
                led.set_high();
            } else {
                println!("input low");
                led.set_low();
            }
            before_level = current_level;
        }
    }

    led.set_low();

    Ok(())
}
