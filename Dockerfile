FROM ubuntu:latest

# Create a non-root user
RUN useradd -m dev
RUN usermod -aG sudo dev

# Get the required packages from apt
RUN apt-get update -y
RUN apt-get install -y curl gcc gcc-arm-none-eabi libclang-dev libc6-dev

# Install rust
RUN su dev -c "curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain nightly --component rust-src clippy cargo rustc rust-std rust-docs rust-analyzer"

WORKDIR /project

CMD su dev -c "bash"
