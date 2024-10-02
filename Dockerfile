FROM rust:latest

WORKDIR /usr/src/calculator

COPY . .

RUN cargo build --release

CMD cargo run