FROM rust:1.94-alpine AS builder

RUN apk add --no-cache \
    build-base \
    openssl-dev \
    mysql-dev \
    pkgconfig \
    musl-dev \
    linux-headers \
    ca-certificates

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release --locked || true

COPY . .
RUN cargo build --release --locked

FROM alpine:3.18

RUN apk add --no-cache \
    ca-certificates \
    mariadb-connector-c \
    openssl

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/settings-service ./settings-service
COPY --from=builder /usr/src/app/migrations ./migrations

RUN adduser -D app && chown -R app:app ./settings-service ./migrations
USER app

EXPOSE 3004

CMD ["./settings-service"]
