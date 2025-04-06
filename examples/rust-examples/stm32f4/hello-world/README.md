# Rust Example for STM32F4 Architecture

Presented here is a straightforward 'Hello world' Rust example utilizing Martos.

Within the setup function, the phrase 'Setup 'hello world' from UART!' is printed once over USART1.
Additionally, within the loop function, the counter value is printed fifty times over semihosting.

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
cargo build --release
```

## How to Run the Example

Option 1: Run in QEMU
If you want to test the program in QEMU, use the following command:

```
qemu-system-arm \
    -M netduinoplus2 \
    -cpu cortex-m4 \
    -nographic \
    -semihosting-config enable=on,target=native \
    -kernel target/thumbv7em-none-eabi/release/example_stm32f4
```
You should see the message "Setup 'hello world' from UART!" and 50 counter values ​​printed in the terminal.