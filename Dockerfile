FROM rust:slim-bullseye
RUN apt-get update && apt-get install -y gcc g++ pkg-config libsasl2-dev libssl-dev librdkafka-dev cmake ninja-build libzstd-dev zlib1g-dev git && rm -rf /var/cache/apt/lists
WORKDIR /app
COPY ./ /app
RUN cargo build --release
RUN strip target/release/libgeyser_neon.so

FROM solanalabs/solana:v1.13.4
RUN apt-get update && apt-get install -y libsasl2-2 && rm -rf /var/cache/apt/lists
COPY --from=0 /app/target/release/libgeyser_neon.so /usr/local/geyser_neon/libgeyser_neon.so
