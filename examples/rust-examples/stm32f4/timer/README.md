# Rust Example for STM32F4 Architecture

Presented here is a straightforward Rust example utilizing Martos with timer usage.

Within the setup function, the phrases 'Initializing timer...' and 'Timer initialized and started.' is printed once.
Additionally, within the loop function, the timer value (last value of dynamic vector) is printed fifty times.

## How to Install Dependencies

Below is a step-by-step guide for installing dependencies on a Linux (Ubuntu/Debian) system.
```
apt-get -qq update
apt-get install -y -q build-essential curl
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
rustup target add thumbv7em-none-eabi
```

## How to build the example
```
cargo build --features="stm32f429" --release
```

## How to Run the Example
Below, you will find an illustrative example showcasing the running on a Linux system (Ubuntu/Debian):
```
cargo run
```