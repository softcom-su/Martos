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
Due to the fact that qemu is currently unable to fully emulate a clock ([STM32F4 only, reset and enable only](https://www.qemu.org/docs/master/system/arm/stm32.html)), the operation of this example cannot be viewed on qemu. But the example works on the board.

To run the example on a real board, follow these steps:
1) Launch the first terminal and run OpenOCD:
```
openocd -f openocd.cfg
```
2) Launch second terminal and connect to OpenOCD via GDB:
```
gdb-multiarch -x openocd.gdb target/thumbv7em-none-eabihf/release/example_stm32f4
```
