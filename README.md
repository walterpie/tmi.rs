# tmi-rs

This crate provides somewhat idiomatic Rust bindings to tmi.cxx.

# Instructions

## Building example(-s)

NOTE: this is only confirmed to work on Linux.

1. Install `node` (and `npm`) and `libclang`
2. `git clone` this repository
3. Run `git submodule init` and `git submodule update --recursive`
4. `cd` into `tmi_cxx`
5. Run `npm install`
6. Go back to the root of tmi-rs. Run `cargo build`.
   NOTE: You might have to set the `LIBCLANG_PATH` to the directory that 
   contains libclang.so if you get an error message.
7. Run `cargo build --example bot`
8. Create a `config.json` and a `secret.json` for your bot.  The format is
   described in the [tmi.cxx](https://github.com/walterpie/tmi.cxx) repo.
9. Now you can run the example with `node tmi_cxx/tmi_cxx.js ./target/debug/examples/libbot.so config.json secret.json`
