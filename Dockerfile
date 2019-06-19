############################################################
# Dockerfile to build Wasabi container
# Based on Ubuntu
############################################################

FROM rust:1.31.1 as build

RUN USER=root cargo new --bin wasabi
WORKDIR /wasabi

# Copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./lib ./lib

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source tree
COPY ./src ./src

# Build for release
RUN cargo build --release

# Final base
FROM debian:jessie-slim

# Copy build artifact
COPY --from=build /wasabi/target/release/wasabi .

# Startup command
ENTRYPOINT ["./wasabi"]
