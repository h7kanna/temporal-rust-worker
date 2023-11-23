# Temporal Rust SDK example

**NOTE**: This is not the official Rust SDK for Temporal, but based on a [fork](https://github.com/h7kanna/sdk-core)
by [Harsha Teja Kanna](https://www.ekalavya.dev/).

**NOTE**: This is a work in progress and experimental. The API is not stable and is subject to change.

# Prerequisites

- Rust https://www.rust-lang.org/tools/install
- Cargo https://doc.rust-lang.org/cargo/getting-started/installation.html
- Temporal Server https://docs.temporal.io/application-development/foundations#run-a-development-server

# Build

```shell
cargo build
```

# Docker build
```shell
docker build --build-arg="RUST_TARGET_ARCH=aarch64-unknown-linux-gnu" -t h7kanna/temporal-rust-worker .
```

# Run

```shell
temporal server start-dev
```

```shell
cargo run # connects to port 7233 from temporal-frontend
```

In a separate terminal:

```shell
temporal workflow start --task-queue="example-task-queue" --type="sdk_example_workflow" --workflow-id="example_id" --input='{"code":"rust","kind":"typesafe"}'

```

Check the execution in the Temporal Web UI: http://localhost:8233/namespaces/default/workflows

Check out https://www.ekalavya.dev/ for more upcoming blogs on Temporal Rust SDK.
