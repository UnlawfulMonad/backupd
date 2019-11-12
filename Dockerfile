FROM rust:1.39.0 AS build

RUN mkdir -p /code && \
    apt-get update && \
    apt-get install -y libsqlcipher-dev sqlite3
WORKDIR /code

COPY . /code
RUN cargo build --release --all

FROM debian:buster
RUN apt-get update && \
    apt-get install -y libsqlcipher0 openssl && \
    apt-get clean -y

COPY --from=build /code/target/release/server /server
COPY --from=build /code/target/release/agent /agent
COPY scripts/docker-entrypoint.sh /docker-entrypoint.sh
CMD ["/bin/sh", "/docker-entrypoint.sh"]
