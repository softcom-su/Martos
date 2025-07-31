# Martos C static library

This README provides detailed instructions on how to build the Martos C static library independently, should you wish to do so.

## How to install dependencies

Below is a step-by-step guide for installing dependencies on a Linux (Ubuntu/Debian) system.
```
apt-get -qq update
apt-get install -y -q build-essential curl
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
rustup target add thumbv7em-none-eabihf
```

## How to build the library

```
cargo build --features="stm32f429" --release
```
