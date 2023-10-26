# syntax=docker/dockerfile:1

FROM alpine:latest
WORKDIR /usr/bin
COPY ./target/x86_64-unknown-linux-musl/release/education-manager /usr/bin/
CMD ["education-manager"]
EXPOSE 8080
