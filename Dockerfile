FROM ubuntu:latest

# Install QEMU
RUN apt-get update && \
    apt-get install qemu-system && 

# Install Rust and other dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl && \
    rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH="$PATH:/root/.cargo/bin"
RUN rustup component add rust-src --toolchain nightly
RUN rustup component add llvm-tools-preview --toolchain nightly
RUN cargo install bootimage
