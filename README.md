# Rust LM75 Temperature Sensor and Thermal Watchdog Driver

[![crates.io](https://img.shields.io/crates/v/lm75.svg)](https://crates.io/crates/lm75)
[![Docs](https://docs.rs/lm75/badge.svg)](https://docs.rs/lm75)
[![Build Status](https://travis-ci.org/eldruin/lm75-rs.svg?branch=master)](https://travis-ci.org/eldruin/lm75-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/lm75-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/lm75-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

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

This driver is also compatible with at least [LM75A], [LM75B, LM75C],
[AT30TS75A], [DS1775], [DS75], [DS7505], [G751], [MAX7500/1/2/3/4], [MAX6625], [MCP9800/1/2/3],
[STDS75], [TCN75].

[AT30TS75A]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8839-DTS-AT30TS75A-Datasheet.pdf
[DS1775]: https://datasheets.maximintegrated.com/en/ds/DS1775-DS1775R.pdf
[DS75]: https://datasheets.maximintegrated.com/en/ds/DS75.pdf
[DS7505]: https://datasheets.maximintegrated.com/en/ds/DS7505.pdf
[G751]: http://www.gmt.com.tw/product/datasheet/EDS-751.pdf
[LM75A]: https://www.nxp.com/docs/en/data-sheet/LM75A.pdf
[LM75B, LM75C]: http://www.ti.com/lit/ds/symlink/lm75b.pdf
[MAX6625]: https://datasheets.maximintegrated.com/en/ds/MAX6625-MAX6626.pdf
[MAX7500/1/2/3/4]: https://datasheets.maximintegrated.com/en/ds/MAX7500-MAX7504.pdf
[MCP9800/1/2/3]: http://ww1.microchip.com/downloads/en/DeviceDoc/21909d.pdf
[STDS75]: https://www.st.com/resource/en/datasheet/stds75.pdf
[TCN75]: http://ww1.microchip.com/downloads/en/DeviceDoc/21490D.pdf

### Usage

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
extern crate linux_embedded_hal as hal;
extern crate lm75;

use hal::I2cdev;
use lm75::{ Lm75, SlaveAddr };

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Lm75::new(dev, address);
    let temp_celsius = sensor.read_temperature().unwrap();
    println!("Temperature: {}ºC", temp_celsius);
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/lm75-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

