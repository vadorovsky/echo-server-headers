FROM rust:alpine3.15 as build

COPY . /src
WORKDIR /src
RUN apk add \
    gcc \
    musl-dev
RUN cargo build --release

FROM alpine:3.15
COPY --from=build /src/target/release/echo-server-headers /usr/bin/echo-server-headers
ENTRYPOINT ["echo-server-headers"]