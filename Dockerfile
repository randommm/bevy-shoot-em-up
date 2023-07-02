FROM ubuntu

RUN apt-get update

RUN apt-get install -y \
    curl \
    clang \
    gcc \
    g++ \
    zlib1g-dev \
    libmpc-dev \
    libmpfr-dev \
    libgmp-dev \
    git \
    cmake \
    pkg-config \
    libssl-dev \
    build-essential

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s - -y

ENV PATH=/root/.cargo/bin:${PATH}

WORKDIR /opt

RUN rustup target install wasm32-unknown-unknown

RUN mkdir out && cd out && \
    curl https://bevyengine.org/tools.js -O && \
    curl https://bevyengine.org/restart-audio-context.js -O

COPY Cargo.toml Cargo.toml

COPY Cargo.lock Cargo.lock

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release --locked --target wasm32-unknown-unknown

RUN rm -rf src

COPY src src

RUN cargo run --release --locked --target wasm32-unknown-unknown

RUN wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy-test.wasm
