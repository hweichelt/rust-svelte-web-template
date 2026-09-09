# myapp

A full-stack web app template: Rust API with database-backed session
authentication, SvelteKit frontend with shadcn-svelte, PostgreSQL, and a
Docker Compose setup that runs all three.

## Using this template

`myapp` is a placeholder. Rename it once, everywhere, before you start:

```sh
git ls-files -z | xargs -0 sed -i 's/myapp/yourname/g'
sed -i 's/myapp/yourname/g' .env
```

That covers the crate name (`Cargo.toml`), the tracing target in `RUST_LOG`,
the Postgres credentials, the compose container names, the session cookie
name on both sides (`src/auth/session.rs`, `frontend/src/lib/server/session.ts`),
and the page titles. Rebuild afterwards so `Cargo.lock` picks up the new
crate name.

- `/` (this crate): the API. Rust, [Axum](https://github.com/tokio-rs/axum), [SeaORM](https://www.sea-ql.org/SeaORM/) 2.0, PostgreSQL.
- `frontend/`: the web app. SvelteKit (Svelte 5), Tailwind 4, [shadcn-svelte](https://shadcn-svelte.com).

## Run everything with Docker

```sh
cp .env.example .env                 # one env file for db, backend and frontend
docker compose up -d --build
```

| Service    | URL                     |
|------------|-------------------------|
| frontend   | http://localhost:8080   |
| backend    | http://localhost:3001   |
| postgres   | 127.0.0.1:5434          |

The browser only talks to the frontend. The SvelteKit server calls the backend
over the compose network and re-issues the session cookie on its own origin.

## Local development

```sh
cp .env.example .env                 # adjust if needed
docker compose up -d db              # just Postgres
cargo run                            # applies migrations, listens on 127.0.0.1:3000

cd frontend
npm install
npm run dev                          # http://localhost:5173
```

Migrations are applied automatically on backend startup.

## Common commands

| Task                                | Command                                                            |
|-------------------------------------|--------------------------------------------------------------------|
| Run the server                      | `cargo run`                                                        |
| Run tests (needs the database)      | `cargo test`                                                       |
| Lint / format                       | `cargo clippy --workspace --all-targets` / `cargo fmt --all`       |
| Create a migration                  | `sea-orm-cli migrate generate <name>`                              |
| Apply / roll back migrations        | `sea-orm-cli migrate up` / `sea-orm-cli migrate down`              |
| Regenerate entities from the schema | `sea-orm-cli generate entity -o src/entities --with-serde serialize --date-time-crate chrono` |

Entities in `src/entities/` are generated. Do not edit them by hand; change the
schema with a migration and regenerate.

## Configuration

One `.env` in the repo root configures everything. The backend, `sea-orm-cli`,
the frontend dev server (via `envDir: '..'` in `frontend/vite.config.ts`) and
docker compose all read it. Its values target running on your machine; compose
overrides the container-network addresses inline in `docker-compose.yml`.

| Variable             | Default                  | Used by            | Purpose                                            |
|----------------------|--------------------------|--------------------|----------------------------------------------------|
| `POSTGRES_USER`      | `myapp`                 | compose, `DATABASE_URL` | Database credentials, defined once           |
| `POSTGRES_PASSWORD`  | `myapp`                 | compose, `DATABASE_URL` |                                              |
| `POSTGRES_DB`        | `myapp`                 | compose, `DATABASE_URL` |                                              |
| `POSTGRES_HOST_PORT` | `5434`                   | compose            | Host port for Postgres                             |
| `BACKEND_HOST_PORT`  | `3001`                   | compose            | Host port for the backend container                |
| `FRONTEND_HOST_PORT` | `8080`                   | compose            | Host port for the frontend container               |
| `FRONTEND_ORIGIN`    | `http://127.0.0.1:8080`  | compose            | Exact URL users open the compose frontend at. SvelteKit rejects form posts from other origins; also the backend's CORS origin. Set to your public URL when deploying |
| `DATABASE_URL`       | built from the above     | backend            | Postgres connection string                         |
| `BIND_ADDR`          | `127.0.0.1:3000`         | backend            | Listen address                                     |
| `CORS_ORIGINS`       | `http://localhost:5173`  | backend            | Comma-separated allowed origins (credentials on)   |
| `SESSION_TTL_DAYS`   | `30`                     | backend            | Session lifetime                                   |
| `COOKIE_SECURE`      | `false`                  | backend, frontend  | `Secure` flag on the session cookie; `true` behind HTTPS |
| `RUST_LOG`           | `info`                   | backend            | Log filter                                         |
| `API_URL`            | `http://127.0.0.1:3000`  | frontend           | Backend base URL as seen from the SvelteKit server |

Only variables prefixed with `VITE_` are exposed to browser code.

## API

Errors are always JSON: `{"error": {"code": "<code>", "message": "<text>"}}`.

### Auth

Sessions are stored in the database. The client gets an opaque token in the
`myapp_session` cookie (HttpOnly, SameSite=Lax); the API also accepts it as
`Authorization: Bearer <token>`.

| Method | Path                 | Body                                  | Response                    |
|--------|----------------------|---------------------------------------|-----------------------------|
| POST   | `/api/auth/register` | `{email, display_name, password}`     | `201` user, sets cookie. `409` if the email exists |
| POST   | `/api/auth/login`    | `{email, password}`                   | `200` user, sets cookie. `401` on bad credentials |
| POST   | `/api/auth/logout`   |                                       | `204`, clears cookie        |
| GET    | `/api/auth/me`       |                                       | `200` user, `401` if not signed in |
| GET    | `/health`            |                                       | `200` when the database answers, `503` otherwise |

User objects look like `{id, email, display_name, created_at}`.

## Frontend

```sh
cd frontend
npm run dev        # dev server
npm run check      # svelte-check
npm run lint       # prettier + eslint
npm run build      # production build (adapter-node), run with `node build`
npx shadcn-svelte@latest add <component>   # add UI components
```

Routes:

| Path         | Purpose                                                        |
|--------------|----------------------------------------------------------------|
| `/`          | Signed-in home (redirects to `/login` otherwise)               |
| `/login`     | Login form                                                     |
| `/register`  | Registration form                                              |
| `/logout`    | POST-only form action that ends the session                    |
| `/api/*`     | Same-origin proxy to the backend for browser-side fetches      |

Auth flow: form actions on the SvelteKit server call the backend, read the
session token from its `Set-Cookie`, and set it as the frontend's own
`myapp_session` cookie. `hooks.server.ts` resolves the user once per request
via `/api/auth/me` and exposes it as `locals.user`.

Configuration comes from the root `.env` (`API_URL`, `COOKIE_SECURE`); the
production Node server additionally takes `ORIGIN` and `PORT`, which compose
derives from `FRONTEND_ORIGIN`.

## Layout

```
migration/          SeaORM migration crate (workspace member)
src/main.rs         startup: config, db, migrations, server
src/lib.rs          module tree + db connection
src/config.rs       environment configuration
src/state.rs        shared AppState
src/error.rs        AppError -> JSON error responses
src/extract.rs      JSON extractor with JSON-shaped rejections
src/auth/           password hashing, db-backed sessions, CurrentUser extractor
src/routes/         axum routers and handlers
src/entities/       generated SeaORM entities
tests/              integration tests against a real database
frontend/           SvelteKit app (see above)
Dockerfile          backend image; frontend/Dockerfile builds the web app
docker-compose.yml  db + backend + frontend
```
