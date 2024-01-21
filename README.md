# b2

b2 is a simple bootloader. It supports x86-64 with UEFI now. Other platforms with UEFI is not tested, but there should be no need to introduce too many changes.

b2 is not ready for production usage, so it's suggested to have a backup bootloader for now.

## Build

`Cargo` is used for building. If you don't have this installed, it's simple to install with `rustup` at <https://rustup.rs/>. 

On *nix systems, run the following in your terminal.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install rust-nightly toolchain and UEFI target.

```
rustup toolchain install nightly-x86_64-unknown-linux-gnu
rustup target install x86_64-unknown-uefi
```

Now build b2. To build a release (some debug features will not be compiled), append `--release` to this command. Append `--profile release-compact` to build a compact binary.

```
cargo b --release --target x86_64-unknown-uefi 
```

## Features
Features is gated through Cargo features and build type.

For features enabled through Cargo, check generated rust doc.

Build Types:
 * `debug`: Retains debug information, keep b2's debug features. No optimization for speed and size.
 * `release`: Debug features is not available in this build.
 * `release-compact`: Like `release`, and there are further optimization to binary size.