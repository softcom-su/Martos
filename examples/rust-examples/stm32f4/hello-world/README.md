# Rust Example for STM32F4 Architecture

Presented here is a straightforward 'Hello world' Rust example utilizing Martos.

The program initializes the UART peripheral on STM32F4 and continuously sends the message "Hello, world\r" over USART1.

## How to Install Dependencies

Below is a step-by-step guide for installing dependencies on a Linux (Ubuntu/Debian) system.
```
apt-get -qq update
apt-get install -y -q build-essential curl
curl https://sh.rustup.rs -sSf | sh -s -- -y
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
    -kernel target/thumbv7em-none-eabi/debug/example_stm32f4
```
You should see the message "Hello, world" printed repeatedly in the terminal.
