#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, Direction, I16x3};

#[entry]
fn main() -> ! {
    let (mut leds, mut lsm303dlhc, mut delay, mut _itm) = aux15::init();

    loop {
        let I16x3 { y, .. } = lsm303dlhc.mag().unwrap();

        
        match y>0 {
             true => iprintln!(&mut _itm.stim[0], "Face down"),
             false => iprintln!(&mut _itm.stim[0], "Face up"),
        }

        delay.delay_ms(1_000_u16);
    }
}
