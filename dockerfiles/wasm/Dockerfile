FROM emscripten/emsdk:4.0.1-arm64

ENV BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/emsdk/upstream/emscripten/cache/sysroot"
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN apt update && apt install -y clang

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && rustup target add wasm32-unknown-unknown

COPY ./Cargo.toml ./Cargo.lock /app/
