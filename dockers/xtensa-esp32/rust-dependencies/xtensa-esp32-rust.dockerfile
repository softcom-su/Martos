FROM ubuntu:latest

# Update default packages
RUN apt-get -qq update
# Get Ubuntu packages
RUN apt-get install -y -q build-essential curl
# Get Rust; NOTE: using sh for better compatibility with other base images
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"
# Install dependencies
# RUN cargo install espup - The official release has not yet updated zip v3.0.0 (https://github.com/esp-rs/espup/commits/main/), and the previous version has been yanked
RUN cargo install --git https://github.com/esp-rs/espup.git espup
RUN espup install
RUN rustup toolchain list
