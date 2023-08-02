# muma-api

The integration api with muma.

## Environment

Configure your `.env` to include the following environment variables:

```bash
DATBASE_URL=""
```

## Development

Generate local certs for ssl and `https`:

```bash
mkcert -install -cert-file ./cert.pem -key-file ./key.pem localhost
```