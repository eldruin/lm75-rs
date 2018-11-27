# Rust LM75 Temperature Sensor and Thermal Watchdog Driver [![crates.io](https://img.shields.io/crates/v/lm75.svg)](https://crates.io/crates/lm75) [![Docs](https://docs.rs/lm75/badge.svg)](https://docs.rs/lm75) [![Build Status](https://travis-ci.org/eldruin/lm75-rs.svg?branch=master)](https://travis-ci.org/eldruin/lm75-rs)

This is a platform agnostic Rust driver for the LM75 temperature sensor
and thermal watchdog, based on the
[`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.

This driver allows you to:
- Enable/disable the device.
- Read the temperature.
- Set the fault queue.
- Set the OS temperature.
- Set the hysteresis temperature.
- Set the OS operation mode.
- Set the OS polarity.

## The device
The LM75 temperature sensor includes a delta-sigma analog-to-digital
converter, and a digital overtemperature detector. The host can
query the LM75 through its I2C interface to read temperature at any
time. The open-drain overtemperature output (OS) sinks current when
the programmable temperature limit is exceeded.
The OS output operates in either of two modes, comparator or
interrupt. The host controls the temperature at which the alarm is
asserted (TOS) and the hysteresis temperature below which the alarm
condition is not valid (THYST). Also, the LM75's TOS and THYST
registers can be read by the host. The address of the LM75 is set
with three pins to allow multiple devices to work on the same bus.
Power-up is in comparator mode, with defaults of TOS= +80ºC and
THYST= +75ºC. The 3.0V to 5.5V supply voltage range, low supply
current, and I2C interface make the LM75 ideal for many applications
in thermal management and protection.

Datasheet:
- [LM75](https://datasheets.maximintegrated.com/en/ds/LM75.pdf)

This driver is also compatible with LM75A, LM75B and LM75C: [LM75B/C Datasheet]

[LM75B/C Datasheet]: http://www.ti.com/lit/ds/symlink/lm75b.pdf

And also at least with the devices MAX7500, MAX6625, MAX6626, DS75LV,
and DS7505.

Please find additional examples in this repository: [lm75-examples]

[lm75-examples]: https://github.com/eldruin/lm75-examples

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

