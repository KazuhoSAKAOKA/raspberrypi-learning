use std::error::Error;
use rppal::spi;
use rppal::spi::{Spi, Bus, SlaveSelect};

fn main()-> Result<(), Box<dyn Error>> {
    println!("spi");

    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 6_2500_000, spi::Mode::Mode0)
            .expect("Failed Spi::new");
    let write_data = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    let mut read_data = [0; 8];
    let read_size = spi.transfer( &mut read_data, &write_data ).expect( "Failed Spi::transfer" );

    for i in 0..read_size {
        println!( "read_data[{}] = 0x{:X}", i, read_data[i] );
    }

    Ok(())
}
