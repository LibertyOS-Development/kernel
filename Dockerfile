# podman build -t los:latest -f Dockerfile .

#FROM rust:1.57-alpine
FROM rust:alpine

RUN apk add --no-cache musl-dev

RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup default nightly
RUN rustup component add \
	rust-src \
	llvm-tools-preview
RUN cargo install bootimage

# bring in the code from your git clone
COPY . .
# put the image in a known location
RUN cargo install --debug --root /usr/local --path .

# A container must be created before extracting the kernel.
# los=$(podman create --name mylos los:latest)
# podman cp $los:/usr/local/bin/libertyos_kernel .
