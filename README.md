# bit-bang-serial

> Bit banging implementation of serial communication using embedded-hal `OutputPin`

## Limitations

- 8N1 Tx Only (No Rx)
- Only tested on [STM32F3DISCOVERY](https://github.com/japaric/f3) and [BETAFPV-F3](https://github.com/JoshMcguigan/betafpv-f3) 
- Must be compiled in `release` mode for correct timing

## Example

An example for the BETAFPV-F3 is listed below. Be sure to compile in release mode.

```rust
    let Board {motor_1: mut out, mut delay, ..} = Board::new();

    let baud = 9600u32;
    let time_adjustment = -6i32;
    let mut tx = bit_bang_serial::Tx::new(out, baud, time_adjustment);

    loop {
        tx.write(&mut delay, [true, true, true, true, false, true, true, false]);
        delay.delay_ms(5u32);
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
