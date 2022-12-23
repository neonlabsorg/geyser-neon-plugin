FROM rust:1.65.0-slim-buster
RUN apt-get update && apt-get install -y gcc g++ pkg-config libsasl2-dev libssl-dev librdkafka-dev cmake ninja-build libzstd-dev zlib1g-dev git && rm -rf /var/cache/apt/lists
WORKDIR /app
COPY ./ /app
# This is for displaying commit hash and branch
COPY .github /app
RUN cargo build --release
RUN strip target/release/libgeyser_neon.so

# TODO: use different version here
FROM neonlabsorg/neon-validator:v1.13.4-plugin-v2
RUN apt-get update && apt-get install -y libsasl2-2 && rm -rf /var/cache/apt/lists
COPY --from=0 /app/target/release/libgeyser_neon.so /opt/solana/bin/libgeyser_neon.so
