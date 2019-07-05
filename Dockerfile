FROM rust:1.36.0 AS build

RUN mkdir -p /code
WORKDIR /code

COPY . /code
RUN cargo build --release --all

FROM debian:stretch
RUN apt-get update && \
    apt-get install -y openssl && \
    apt-get clean -y

COPY --from=build /code/target/release/server /server
COPY --from=build /code/target/release/agent /agent
COPY scripts/docker-entrypoint.sh /docker-entrypoint.sh
CMD ["/bin/sh", "/docker-entrypoint.sh"]
