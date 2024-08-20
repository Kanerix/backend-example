# Lerpz Backend

The backend API for the Lerpz website.

## Setup

### Docker

- Run the command `docker run lerpz_backend -t lerpz_backend -v keys-docker:/app/var/keys:ro -p 3000:3000`

### Local

- Install `rust` programming language

- Setup ED25519 keys for JWT token signing.

```bash
openssl genpkey -algorithm ED25519 -outform PEM -out ./keys/ed25519_private.pem 
openssl pkey -in ./keys/ed25519_private.pem -pubout -out ./keys/ed25519_public.pem
```
