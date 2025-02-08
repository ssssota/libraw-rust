FROM ghcr.io/webassembly/wasi-sdk:wasi-sdk-25

ENV BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/opt/wasi-sdk/share/wasi-sysroot"
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN apt update && apt install -y curl clang unzip
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && rustup target add wasm32-wasip1-threads

RUN curl -fsSL https://fnm.vercel.app/install | bash -s -- --install-dir "/usr/local/share/fnm" --skip-shell \
    && ln -s /usr/local/share/fnm/fnm /usr/local/bin/fnm \
    && fnm install 22 \
    && fnm default 22
ENV PATH=/root/.local/share/fnm/aliases/default/bin:$PATH
RUN npm i -g pnpm 

WORKDIR /app
COPY ./Cargo.toml ./Cargo.lock ./package.json ./pnpm-lock.yaml ./pnpm-workspace.yaml /app/
RUN pnpm install --frozen-lockfile
