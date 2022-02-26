FROM rust:1.58

RUN cargo new --bin rust-and-docker
WORKDIR ./rust-and-docker
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build

FROM alpine:latest
ENTRYPOINT [ "./rust-and-docker" ]