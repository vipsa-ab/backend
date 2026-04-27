# Vipsa Backend — Agent Instructions

## Project
- Single Rust workspace: `vipsa-backend/`
- Binary crate named `vipsa_backend`, entrypoint in `src/main.rs`
- Standard Cargo layout, no workspace/monorepo config

## Architecture (Hexagonal)
```
src/
├── domain/       # Pure business logic. ZERO external dependencies.
│   ├── entities/ # User, Notification — no framework structs
│   ├── repositories/  # Traits: Repository<T>, Entity — defined here, implemented in infrastructure
│   └── services/     # UserService, NotificationService, EmailPort trait
├── application/  # Thin orchestration (commands/queries), no business logic
├── infrastructure/  # Implements domain ports
│   ├── adapters/
│   │   ├── postgres/   # PostgresUserRepository, PostgresNotificationRepository
│   │   └── resend/     # ResendEmailAdapter (implements EmailPort)
│   ├── config/     # AppConfig::load() — looks for config.toml, config.local.toml, env vars with VIPSA__ prefix
│   └── mappers/    # entity_to_dto.rs (EntityToDto struct with methods), dto_to_domain.rs (module functions)
└── api/           # HTTP layer: DTOs, routes, handlers
```

## Critical Patterns

### Port/Adapter pattern
- **Domain defines** `Repository<T>` trait and `EmailPort` trait
- **Infrastructure implements** them — swap implementations without touching domain
- In `main.rs`, inject via `Arc<dyn Trait>`:
  ```rust
  let user_repository = Arc::new(PostgresUserRepository::new(pool));
  let email_adapter = Arc::new(ResendEmailAdapter::new(...));
  let user_service = Arc::new(UserService::new(user_repository));
  ```

### Mapper pattern
- `entity_to_dto.rs` has `EntityToDto` struct with `to_user_dto()` / `to_notification_dto()` methods
- `dto_to_domain.rs` has module-level functions like `create_user_from_dto()`
- Domain types NEVER leak to API layer — mappers are the boundary

### Tests
- Unit tests live **inline** in service files (`src/domain/services/user.rs`, `notification.rs`)
- Use manual mocks with `Mutex` (not mockall trait), implementing `Repository<T>`
- Run single test file: `cargo test --lib domain::services::user`
- Run all tests: `cargo test`

## Dev Commands
```bash
cargo build          # Compile
cargo test           # Run tests
cargo check          # Type-check without building
cargo run            # Run dev server (requires DATABASE_URL env var or config file)
```

## Config
- Looks for `config.toml` or `config.local.toml` in project root
- Environment variables: `VIPSA__DATABASE__URL`, `VIPSA__EMAIL__RESEND_API_KEY`, etc.
- Defaults if no config: localhost:5432/postgres, no email API key

## Database
- sqlx with PostgreSQL, uses compile-time query checking (prepare)
- Migrations not set up yet — use `sqlx migrate add <name>` when needed
- Requires `DATABASE_URL` environment variable or config entry

## Adding New Services
1. Define entity in `domain/entities/`
2. Define port trait in `domain/repositories/` or `domain/services/`
3. Implement adapter in `infrastructure/adapters/<name>/`
4. Add mapper in `infrastructure/mappers/`
5. Add routes in `api/routes/`

## What's NOT Here
- No CI/CD workflows yet
- No pre-commit hooks
- No OpenCode config (`opencode.json`) — create if needed
- No README — write when project stabilizes