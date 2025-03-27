# Bevy Mod GameBoy Advance

This crate provides integration between the [`agb`](https://crates.io/crates/agb) HAL for the GameBoy Advance, and the [Bevy game engine](https://crates.io/crates/bevy).

Simply add the `AgbPlugin` to your `no_std` Bevy application, and you'll have access to:

* The gamepad using Bevy's idiomatic `Gamepad` component
* A basic renderer providing a `Sprite` component
* Integration with `Time` and the built-in hardware timer
* A custom application runner chasing V-Blank
* Logging integration when using the mGBA emulator

Below is a screenshot from the `game` example on the repository.

![Demo from `examples/game.rs`](assets/game_capture.gif)

## Building

### Recommended prerequisites

You may want to install the below to allow running the example:

* A GameBoy Advance emulator. The best support in agb is with [mgba](https://mgba.io). Ensure `mgba` is in your `PATH` for the best experience.
* `agb-gbafix` using `cargo install agb-gbafix`. This is not required if you are only running your game in an emulator, but does allow neatly packaging a ROM.
* Rust's nightly toolchain.

### Setup

Since this is largely based on [`abg`](https://github.com/agbrs/agb) it's possible to get started using their [template](https://github.com/agbrs/template).
If you'd like to setup a project from scratch, here's what you'll want to do:

1. Create a new project with `cargo init`.
2. Create a `rust-toolchain.toml` file in the project root with the following contents:

   ```toml
   [toolchain]
   channel = "nightly"
   components = ["rust-src", "clippy", "rustfmt"]
   ```

   Note that this informs `cargo` that we require a `nightly` toolchain and would like a copy of the Rust source code, and the `clippy` and `rustfmt` tools.

3. Create a `.cargo/config.toml` file with the following contents:

   ```toml
   [unstable]
   build-std = ["core", "alloc"]
   build-std-features = ["compiler-builtins-mem"]
   
   [build]
   target = "thumbv4t-none-eabi"
   
   [target.thumbv4t-none-eabi]
   rustflags = [
     "-Clink-arg=-Tgba.ld",
     "-Ctarget-cpu=arm7tdmi",
     "-Cforce-frame-pointers=yes",
   ]
   runner = ["mgba", "-C", "logToStdout=1", "-C", "logLevel.gba.debug=127"]
   
   [target.armv4t-none-eabi]
   rustflags = [
     "-Clink-arg=-Tgba.ld",
     "-Ctarget-cpu=arm7tdmi",
     "-Cforce-frame-pointers=yes",
   ]
   runner = ["mgba", "-C", "logToStdout=1", "-C", "logLevel.gba.debug=127"]
   ```

   This informs the compiler that we need to compile `core` and `alloc` from source, since we have some custom flags to apply.
   Additionally, this sets up the mGBA emulator as our runner, allowing `cargo run` to work as expected.

### Running in an emulator

Once you have the prerequisites installed, you should be able to build using

```sh
cargo build
```

or in release mode (strongly recommended)

```sh
cargo build --release
```

The resulting file will be in `target/thumbv4t-none-eabi/debug/my_game` or `target/thumbv4t-none-eabi/release/my_game` depending on
whether you did a release or debug build.

If you have `mgba` in your path, you will be able to run your game with

```sh
cargo run
```

or in release mode

```sh
cargo run --release
```

## Shipping a .gba file for real hardware

To make a game run on real hardware, you will need to convert the built file into a file suitable for
running on the real thing.

First build the binary in release mode using the instructions above, then do the following:

```sh
agb-gbafix target/thumbv4t-none-eabi/release/my_game -o my_game.gba
```
