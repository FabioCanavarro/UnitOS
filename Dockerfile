FROM ubuntu:latest

# Install QEMU
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    qemu-system-x86 qemu-utils && \
    rm -rf /var/lib/apt/lists/*

# Install Rust and components
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl && \
    rm -rf /var/lib/apt/lists/* && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly && \
    export PATH="$PATH:/root/.cargo/bin" && \
    rustup component add rust-src --toolchain nightly

# Install llvm-tools and bootimage
RUN export PATH="$PATH:/root/.cargo/bin" && \
    rustup component add llvm-tools-preview --toolchain nightly && \
    cargo install bootimage
