# C example for STM32F4 architecture

Presented here is a straightforward C example utilizing Martos.

It has empty setup function.
Additionally, within the loop function, the counter value is incremented fifty times.

## How to install dependencies

Below is a step-by-step guide for installing dependencies on a Linux (Ubuntu/Debian) system.
```
apt-get -qq update
apt-get install -y -q build-essential curl
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
rustup target add thumbv7em-none-eabihf
```

## Before building the example

Before proceeding with building the example, it is essential to obtain the Martos C static library 
and [link it](Makefile#L25) with the example code.

You need to get the Martos C static library, for this you can build the Martos C static library independently [here](../../../c-library/stm32f4).


## How to build the example

Below, you will find an illustrative example showcasing the building process on a Linux system (Ubuntu/Debian):
```
make
```

## How to run the example

To run the example on a real board, follow these steps:
1) Launch the first terminal and run OpenOCD:
```
openocd -f openocd.cfg
```
2) Launch second terminal and connect to OpenOCD via GDB:
```
gdb-multiarch -x openocd.gdb main.elf
```
