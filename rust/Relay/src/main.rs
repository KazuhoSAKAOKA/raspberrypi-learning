use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const GPIO_RELAY:u8 = 17;

fn main()->Result<(), Box<dyn Error>> {

    println!("relay and led");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut relay = Gpio::new()?.get(GPIO_RELAY)?.into_output();

    while running.load(Ordering::SeqCst){
        relay.set_high();
        thread::sleep(Duration::from_millis(1000));
        relay.set_low();
        thread::sleep(Duration::from_millis(1000));
    }

    relay.set_low();
    Ok(())
}
