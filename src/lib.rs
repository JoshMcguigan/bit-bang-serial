#![allow(dead_code)]
#![no_std]

extern crate embedded_hal as hal;
use hal::digital::OutputPin;
use hal::blocking::delay::DelayUs;

pub struct Tx<Out> {
    output_pin: Out,
    microseconds_per_bit: u32,
}

impl<Out> Tx<Out>
    where Out: OutputPin,
{
    pub fn new(mut output_pin: Out, baud: u32, time_adjustment: i32) -> Self {
        output_pin.set_high(); // idle state is high
        let microseconds_per_bit = ((1_000_000u32 / baud) as i64 + time_adjustment as i64) as u32;
        Tx { output_pin, microseconds_per_bit }
    }

    pub fn write(&mut self, delay: &mut DelayUs<u32>, data: [bool; 8]) {
        // set low for start bit
        self.output_pin.set_low();
        delay.delay_us(self.microseconds_per_bit);

        // data bits
        for &bit in data.iter() {
            if bit { self.output_pin.set_high() }
                else { self.output_pin.set_low() }

            delay.delay_us(self.microseconds_per_bit);
        }

        // set high for stop bit
        self.output_pin.set_high();
        delay.delay_us(self.microseconds_per_bit);
    }
}
