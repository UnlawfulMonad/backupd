FROM rust:1.35.0

RUN mkdir -p /code
WORKDIR /code

COPY . /code
RUN cargo build --release --all
