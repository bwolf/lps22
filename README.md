LPS22
=====

[![Latest version](https://img.shields.io/crates/v/lps22.svg)](https://crates.io/crates/lps22)

A platform agnostic driver to interface the [ST][ST] LPS22 pressure sensor, written in Rust. Although the driver is verified using a [`LPS22HH`][LPS22HH] sensor, it should be in theory compatible to other `LPS22` sensors like the [`LPS22HB`][LPS22HB] and if no, it should be easy to make it so.

This driver is build using the [embedded-hal][embedded-hal] traits.

[API reference]

## Features

TODO TBD


## License

Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

[ST]: https://www.st.com/en/mems-and-sensors/pressure-sensors.html?querycriteria=productId=SC1316
[LPS22HH]: https://www.st.com/en/mems-and-sensors/lps22hh.html
[LPS22HB]: https://www.st.com/en/mems-and-sensors/lps22hb.html
[embedded-hal]: https://docs.rs/embedded-hal/
[API reference]: https://docs.rs/lps22
