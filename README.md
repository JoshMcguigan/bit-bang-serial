# bit-bang-serial

> Bit banging implementation of serial communication using embedded-hal `OutputPin`

## Limitations

- 9600 baud 8N1 Tx Only (No Rx)
- Only tested on STM32F3DISCOVERY
- Must be run in `release` mode for correct timing

## Example

A complete, standalone example for the STM32F3DISCOVERY is listed below. Be sure to run with `cargo run --release`.

```rust
    #![deny(unsafe_code)]
    #![no_std]
    #![no_main]
    
    #[macro_use(entry, exception)]
    extern crate cortex_m_rt as rt;
    extern crate cortex_m;
    extern crate f3;
    extern crate panic_semihosting;
    extern crate bit_bang_serial;
    
    use f3::hal::delay::Delay;
    use f3::hal::prelude::*;
    use f3::hal::stm32f30x;
    use rt::ExceptionFrame;
    use cortex_m::asm::bkpt;
    
    entry!(main);
    
    fn main() -> ! {
        bkpt();
    
        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = stm32f30x::Peripherals::take().unwrap();
    
        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
    
        let delay = Delay::new(cp.SYST, clocks);
        let output = gpioa
            .pa9
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    
        let mut tx = bit_bang_serial::Tx::new(output, delay);
    
        loop {
            let data = [true, true, true, true, false, true, true, false];
            tx.write(data); // write the letter 'o'
        }
    }
    
    exception!(HardFault, hard_fault);
    
    fn hard_fault(ef: &ExceptionFrame) -> ! {
        panic!("{:#?}", ef);
    }
    
    exception!(*, default_handler);
    
    fn default_handler(irqn: i16) {
        panic!("Unhandled exception (IRQn = {})", irqn);
    }
```
    
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
