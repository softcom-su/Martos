FROM ubuntu:latest

RUN apt-get -qq update && \
    apt-get install -y -q build-essential curl \
    qemu-system-arm

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add thumbv7em-none-eabi
