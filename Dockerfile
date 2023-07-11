FROM rust:1.55-alpine as builder

run apk add --no-cache musl-dev bash

RUN adduser --disabled-password --gecos "" liberty
USER liberty
WORKDIR /home/liberty
COPY rust-toolchain.toml .

RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup component add rust-src 
RUN rustup component add llvm-tools-preview
RUN cargo install bootimage
RUN rustup default nightly
RUN rustup update nightly

COPY . .
RUN cargo build

RUN touch copy.sh
RUN echo -e "#!/bin/sh\ncp -r ./target/* /mnt/target" > copy.sh

CMD ["/bin/sh", "/home/liberty/copy.sh"]
