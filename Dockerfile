FROM rust:alpine3.18 as chef

RUN apk update && apk upgrade --no-cache
RUN apk add --no-cache musl-dev pkgconf openssl libressl-dev

RUN cargo install cargo-chef


FROM chef AS planner
WORKDIR /plan

COPY rust-toolchain.toml Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
WORKDIR /build

COPY --from=planner /plan/recipe.json ./recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN mkdir keys && \
    openssl genpkey -algorithm ED25519 -outform PEM -out ./keys/ed25519_private.pem && \
    openssl pkey -in ./keys/ed25519_private.pem -pubout -out ./keys/ed25519_public.pem

ENV SQLX_OFFLINE=true

RUN cargo build --release


FROM alpine:3.18 AS runtime
WORKDIR /var/app

COPY --from=builder /build/target/release/backend ./
COPY --from=builder /build/keys ./keys
COPY --from=builder /build/migrations ./migrations

RUN addgroup -S app && \
    adduser -S lerpz -G app && \
    chown -R lerpz:app /var/app

USER lerpz

EXPOSE 8080

ENTRYPOINT ["/var/app/backend"]