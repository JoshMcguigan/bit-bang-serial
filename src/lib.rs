#![allow(dead_code)]
#![no_std]

extern crate embedded_hal as hal;
use hal::digital::OutputPin;
use hal::blocking::delay::DelayUs;

const BAUD : u32 = 9600u32;
const DELAY_ADJUSTMENT : u32 = 6; // Manually determined to work on STM32F3DISCOVERY, may be different on other controllers
const MICROSECONDS_PER_BIT : u32 = 1_000_000u32 / BAUD - DELAY_ADJUSTMENT;

pub struct Tx<Out, Delay> {
    output_pin: Out,
    delay: Delay
}

impl<Out, Delay> Tx<Out, Delay>
    where Out: OutputPin,
        Delay: DelayUs<u32>
{
    pub fn new(mut output_pin: Out, delay: Delay) -> Self {
        output_pin.set_high(); // idle state is high

        Tx { output_pin, delay }
    }

    pub fn write(&mut self, data: [bool; 8]) {
        // set low for start bit
        self.output_pin.set_low();
        self.delay.delay_us(MICROSECONDS_PER_BIT);

        // data bits
        for &bit in data.iter() {
            if bit { self.output_pin.set_high() }
                else { self.output_pin.set_low() }

            self.delay.delay_us(MICROSECONDS_PER_BIT);
        }

        // set high for stop bit
        self.output_pin.set_high();
        self.delay.delay_us(MICROSECONDS_PER_BIT);
    }
}
