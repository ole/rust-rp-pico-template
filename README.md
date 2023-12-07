# Rust RP Pico Template

A template for [cargo-generate](https://github.com/cargo-generate/cargo-generate) that aims to be a starting point for embedded Rust development on the Raspberry Pi Pico. Includes a suitable Visual Studio Code configuration for building, flashing, running, and debugging from VS Code.

## Requirements

Install these Rust components:

```sh
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs --locked
cargo install probe-rs --features cli --locked
cargo install cargo-binutils
```

For VS Code, install these extensions:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer): The official Rust language support for VS Code
- [Debugger for probe-rs](https://marketplace.visualstudio.com/items?itemName=probe-rs.probe-rs-debugger): Debugging support via [probe-rs](https://probe.rs/)
- [Serial Monitor](https://marketplace.visualstudio.com/items?itemName=ms-vscode.vscode-serial-monitor) (optional): Send and receive messages to/from serial ports. Useful for simple logging from your embedded program.
