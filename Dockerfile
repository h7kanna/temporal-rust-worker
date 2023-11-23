FROM --platform=$TARGETPLATFORM rust:1.74.0-slim AS chef
ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN cargo install cargo-chef
WORKDIR /usr/src/temporal-rust-worker

FROM chef AS planner
ARG TARGETPLATFORM
ARG BUILDPLATFORM
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM
ARG RUST_TARGET_ARCH=aarch64-unknown-linux-gnu
COPY --from=planner /usr/src/temporal-rust-worker/recipe.json recipe.json
RUN echo "Building on $BUILDPLATFORM for $TARGETPLATFORM"
RUN apt-get update && apt-get install -y protobuf-compiler
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo chef cook --profile release --target ${RUST_TARGET_ARCH} --recipe-path recipe.json
COPY . .
RUN cargo build --release --target ${RUST_TARGET_ARCH}

FROM --platform=$TARGETPLATFORM cgr.dev/chainguard/static
ARG RUST_TARGET_ARCH=aarch64-unknown-linux-gnu
COPY --from=builder /usr/src/temporal-rust-worker/target/${RUST_TARGET_ARCH}/release/temporal-rust-worker /usr/local/bin/temporal-rust-worker
USER 65532:65532

CMD ["temporal-rust-worker"]
