# Lerpz Backend

## Information

The purpose of this project is to learn and document how the diffrent parts
of creating an API for production work. This is not meant to be used as a library
as it is simply just for learning and exploration purposes.

This is a simple REST API that is built using the [Axum](https://crates.io/crates/axum)
framework. This levrages the [Tokio](https://crates.io/crates/tokio) runtime for asyncronous
operations and the [SQLx](https://crates.io/crates/sqlx) crate for database operations.

The goal of this project is to make it available as a public API for me to use for my own projects.
This will be stuff like websites and other applications that usually interact with an API.

## Setup

Pre-requisites:

- [Rust](https://www.rust-lang.org/learn/get-started) programming language.
- [Docker](https://docs.docker.com/get-docker/) for containers.



Setup ED25519 keys for JWT token signing.

```bash
openssl genpkey -algorithm ED25519 -outform PEM -out ./keys/ed25519_private.pem
openssl pkey -in ./keys/ed25519_private.pem -pubout -out ./keys/ed25519_public.pem
```

Install docker and build the container.

```bash
docker build . -t lerpz-backend
```

Start the container on the desired port.

```bash
docker run lerpz-backend -p 3000:3000
```

Optional: bind the `keys` directory to persist keys used by the JWT module.

```bash
docker run lerpz-backend -v ./keys:/app/var/keys:ro -p 3000:3000
```
