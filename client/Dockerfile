FROM registry.gitlab.com/lucas_pl/backupd/base:latest AS build

FROM debian:stretch
RUN apt-get update && \
    apt-get install -y libssl && \
    apt-get clean
COPY --from=build /code/target/release/client /backupd-client
ENTRYPOINT ["/backupd-client"]
