FROM ubuntu:latest

RUN apt-get -qq update && \
    apt-get install -y -q build-essential curl \
    gcc-arm-none-eabi \
    cmake
