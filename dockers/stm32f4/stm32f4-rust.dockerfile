FROM ubuntu:latest

# Update default packages, get Ubuntu packages and install qemu for emulation
RUN apt-get -qq update && \
    apt-get install -y -q build-essential curl \
    qemu-system-arm

# Get Rust and add .cargo/bin to PATH
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add thumbv7em-none-eabi
