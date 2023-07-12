FROM rust:1.70-alpine
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release -Z unstable-options --out-dir /mnt/target
