FROM ubuntu:latest

# Install necessary packages including ca-certificates and build tools
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    qemu-system-x86 qemu-utils \
    curl ca-certificates \
    build-essential && \
    rm -rf /var/lib/apt/lists/*

# Install Rust and components
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly && \
    . $HOME/.cargo/env && \
    rustup component add rust-src --toolchain nightly && \
    rustup component add llvm-tools-preview --toolchain nightly && \
    cargo install bootimage

# Add cargo bin to PATH permanently
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app
