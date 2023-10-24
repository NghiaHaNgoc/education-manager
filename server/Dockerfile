# syntax=docker/dockerfile:1

FROM debian:latest
WORKDIR /app
COPY ./target/release/education-manager /usr/bin/
COPY ./surreal /usr/bin/
COPY ./start-server /usr/bin/
COPY education-manager/ ./education-manager/
# RUN apt-get update && apt-get install -y curl
# RUN curl -sSf https://install.surrealdb.com | sh
CMD ["start-server"]
EXPOSE 8080
