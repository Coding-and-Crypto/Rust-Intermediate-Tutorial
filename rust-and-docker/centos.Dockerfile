FROM rust:1.58 as builder

RUN cargo new --bin rust-and-docker
WORKDIR ./rust-and-docker
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

FROM centos:latest
COPY --from=builder /rust-and-docker/target/release/rust-and-docker ./rust-and-docker
CMD ["./rust-and-docker"]