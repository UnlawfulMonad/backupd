FROM registry.gitlab.com/lucas_pl/backupd/base:latest AS build

FROM debian:stretch
RUN apt-get update && \
    apt-get install -y openssl && \
    apt-get clean
COPY --from=build /code/target/release/server /backupd-server
ENTRYPOINT ["/backupd-server"]
