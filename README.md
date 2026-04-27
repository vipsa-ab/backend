# Vipsa Backend

REST API backend built with Rust using hexagonal architecture.

## Tech Stack

- **Web Framework**: Axum 0.7
- **Database**: PostgreSQL with sqlx
- **Email**: Resend (resend-rs)
- **Runtime**: Tokio

## Architecture

```
src/
├── domain/           # Pure business logic. Zero external dependencies.
│   ├── entities/     # User, Notification
│   ├── repositories/ # Traits: Repository<T>, Entity
│   └── services/     # UserService, NotificationService, EmailPort
├── application/      # Thin orchestration (commands/queries)
├── infrastructure/   # Implements domain ports
│   ├── adapters/     # PostgresUserRepository, ResendEmailAdapter
│   ├── config/       # AppConfig loading
│   └── mappers/      # entity↔dto converters
└── api/              # HTTP layer: routes, DTOs, handlers
```

### Key Patterns

**Port/Adapter**: Domain defines interfaces (`Repository<T>`, `EmailPort`), infrastructure implements them. Swap implementations without touching business logic.

**Mappers**: Never expose domain types externally. DTOs are the boundary.

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check with version |
| GET | `/users` | List all users |
| POST | `/users` | Create user |
| GET | `/users/{id}` | Get user by ID |
| PUT | `/users/{id}` | Update user |
| DELETE | `/users/{id}` | Delete user |
| GET | `/notifications` | List all notifications |
| POST | `/notifications` | Create notification (no email) |
| GET | `/notifications/{id}` | Get notification by ID |
| POST | `/notifications/{id}/send` | Create + send email |

## Setup

```bash
# Install dependencies
cargo build

# Create database tables
# (run migrations or create manually based on domain entities)

# Run
cargo run
```

## Configuration

Configuration is loaded from (in order):
1. `config.toml`
2. `config.local.toml`
3. Environment variables with `VIPSA__` prefix

Example environment variables:
```bash
export VIPSA__DATABASE__URL="postgres://user:pass@localhost/vipsa"
export VIPSA__EMAIL__RESEND_API_KEY="re_xxxxx"
export VIPSA__EMAIL__FROM_EMAIL="noreply@example.com"
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## Project Status

Initial structure with hexagonal architecture. Database adapters are wired but require PostgreSQL tables (`users`, `notifications`) to be created.