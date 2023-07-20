FROM rust:1.71-alpine
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release -Z unstable-options --out-dir /mnt/target
