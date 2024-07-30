# Server

Rust port of [SponsorBlockServer](https://github.com/ajayyy/SponsorBlockServer).

## Differences
| | Original | Port
| :-- | :--- | :--- |
| Lang | TypeScript | Rust
| Framework | Express | Axum
| Database | Postgres, SQLite | Postgres
| Caching | Redis | Redis
| Search Index | Postgres | Postgres, Meilisearch
| Path prefix | `/api` | Default: `/api` (configurable)

## Images
We provide a rootless, immutable [Alpine](./images/alpine) and [Distroless](./images/distroless) (based on Debian) Docker image.

Additionally, an [example docker compose file](./docker-compose.yml) is provided with [best practice security settings](https://cheatsheetseries.owasp.org/cheatsheets/Docker_Security_Cheat_Sheet.html), such as dropping [capabilities](https://man7.org/linux/man-pages/man7/capabilities.7.html).