FROM ghcr.io/webassembly/wasi-sdk:wasi-sdk-25

ENV BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/opt/wasi-sdk/share/wasi-sysroot"
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN apt update && apt install -y git curl clang
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && rustup target add wasm32-wasip1

COPY ./Cargo.toml ./Cargo.lock /app/
