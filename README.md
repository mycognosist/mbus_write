## mbus-write

A simple utility for generating Modbus RTU write_single_register commands.

### Usage

`mbus_write [MODBUS_ADDRESS] [REGISTER] [VALUE]`

E.g. `mbus_write 481 19 256`

The above command translates to: write 256 to register 19 for unit with Modbus address 481.

`mbus_write -h` for help.

### Dependencies

[clap](https://crates.io/crates/clap) - Easily create commandline utilities and parse arguments

[futures](https://crates.io/crates/futures) - An implementation of futures and streams featuring zero allocations, composability, and iterator-like interfaces

[tokio-core](https://crates.io/crates/tokio-core) - Core I/O and event loop primitives for asynchronous I/O in Rust

[tokio-modbus](https://crates.io/crates/tokio-modbus) - A [tokio](https://tokio.rs/)-based Modbus library

[tokio-serial](https://crates.io/crates/tokio-serial) - A serial port implementation for tokio

### ARM Compilation

[Compiling on Raspberry Pi](https://stackoverflow.com/questions/29917513/how-can-i-compile-rust-code-to-run-on-a-raspberry-pi-2)
