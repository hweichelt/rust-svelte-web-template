# myapp

A full-stack web app template: a Rust API with database-backed session
authentication that also serves the SvelteKit frontend (shadcn-svelte) from
the same binary, PostgreSQL, and a Docker Compose setup for both.

## Using this template

`myapp` is a placeholder. Rename it once, everywhere, before you start:

```sh
git ls-files -z | xargs -0 sed -i 's/myapp/yourname/g'
sed -i 's/myapp/yourname/g' .env
```

That covers the crate name (`Cargo.toml`), the tracing target in `RUST_LOG`,
the Postgres credentials, the compose container names, the session cookie
name (`src/auth/session.rs`), and the page titles. Rebuild afterwards so
`Cargo.lock` picks up the new crate name.

- `/` (this crate): the server. Rust, [Axum](https://github.com/tokio-rs/axum), [SeaORM](https://www.sea-ql.org/SeaORM/) 2.0, PostgreSQL. Serves the API and, via [axum-vite](https://github.com/gko/axum-vite), the web app.
- `frontend/`: the web app. SvelteKit (Svelte 5) as a client-rendered single-page app, Tailwind 4, [shadcn-svelte](https://shadcn-svelte.com).

## How the frontend is served

There is one server and one origin. The SvelteKit app is built as a static
bundle (`adapter-static`, `ssr = false`) and the Rust binary serves it:

- **Release builds** embed `frontend/build` into the binary at compile time.
  Nothing else is needed at runtime, not even Node.
- **Debug builds** (`cargo run`) embed nothing. Page and asset requests are
  proxied to the Vite dev server, so you get hot module reloading through the
  backend's port. By default `cargo run` starts Vite itself (`VITE_AUTO_START`).

API routes are matched first; everything else goes to the app.

## Run everything with Docker

```sh
cp .env.example .env                 # one env file for db and app
docker compose up -d --build
```

| Service    | URL                     |
|------------|-------------------------|
| app        | http://localhost:8080   |
| postgres   | 127.0.0.1:5434          |

## Local development

```sh
cp .env.example .env                 # adjust if needed
docker compose up -d db              # just Postgres
(cd frontend && npm install)         # once
cargo run                            # applies migrations, starts Vite, listens on 127.0.0.1:3000
```

Open http://localhost:3000. Migrations are applied automatically on startup.
To run Vite yourself (for example to see its output in a separate terminal),
set `VITE_AUTO_START=false` and run `npm run dev` in `frontend/`.

To try a release build locally:

```sh
(cd frontend && npm run build)       # produces frontend/build
cargo run --release                  # embeds it
```

A release build without `frontend/build` fails early with a message saying so
(`build.rs`).

## Common commands

| Task                                | Command                                                            |
|-------------------------------------|--------------------------------------------------------------------|
| Run the server (and Vite)           | `cargo run`                                                        |
| Release build with embedded app     | `(cd frontend && npm run build) && cargo build --release`          |
| Run tests (needs the database)      | `cargo test`                                                       |
| Lint / format                       | `cargo clippy --workspace --all-targets` / `cargo fmt --all`       |
| Create a migration                  | `sea-orm-cli migrate generate <name>`                              |
| Apply / roll back migrations        | `sea-orm-cli migrate up` / `sea-orm-cli migrate down`              |
| Regenerate entities from the schema | `sea-orm-cli generate entity -o src/entities --with-serde serialize --date-time-crate chrono` |

Entities in `src/entities/` are generated. Do not edit them by hand; change the
schema with a migration and regenerate.

## Configuration

One `.env` in the repo root configures everything. The backend, `sea-orm-cli`,
the Vite dev server (via `envDir: '..'` in `frontend/vite.config.ts`) and
docker compose all read it. Its values target running on your machine; compose
overrides the database address inline in `docker-compose.yml`.

| Variable             | Default                  | Used by            | Purpose                                            |
|----------------------|--------------------------|--------------------|----------------------------------------------------|
| `POSTGRES_USER`      | `myapp`                 | compose, `DATABASE_URL` | Database credentials, defined once           |
| `POSTGRES_PASSWORD`  | `myapp`                 | compose, `DATABASE_URL` |                                              |
| `POSTGRES_DB`        | `myapp`                 | compose, `DATABASE_URL` |                                              |
| `POSTGRES_HOST_PORT` | `5434`                   | compose            | Host port for Postgres                             |
| `APP_HOST_PORT`      | `8080`                   | compose            | Host port for the app container                    |
| `DATABASE_URL`       | built from the above     | backend            | Postgres connection string                         |
| `BIND_ADDR`          | `127.0.0.1:3000`         | backend            | Listen address                                     |
| `SESSION_TTL_DAYS`   | `30`                     | backend            | Session lifetime                                   |
| `COOKIE_SECURE`      | `false`                  | backend            | `Secure` flag on the session cookie; `true` behind HTTPS |
| `RUST_LOG`           | `info`                   | backend            | Log filter                                         |
| `VITE_PORT`          | `5173`                   | backend (debug), Vite | Port of the Vite dev server; the backend proxies to it |
| `VITE_AUTO_START`    | `true`                   | backend (debug)    | Start `npm run dev` from `cargo run`               |
| `FRONTEND_DIR`       | `frontend`               | backend (debug)    | Where to start Vite, relative to the working directory |

Only variables prefixed with `VITE_` are exposed to browser code.

## API

Errors are always JSON: `{"error": {"code": "<code>", "message": "<text>"}}`.

### Auth

Sessions are stored in the database. The client gets an opaque token in the
`myapp_session` cookie (HttpOnly, SameSite=Lax); the API also accepts it as
`Authorization: Bearer <token>`. The app and the API share an origin, so there
is no CORS layer; `SameSite=Lax` plus JSON-only request bodies keep cross-site
requests out.

| Method | Path                 | Body                                  | Response                    |
|--------|----------------------|---------------------------------------|-----------------------------|
| POST   | `/api/auth/register` | `{email, display_name, password}`     | `201` user, sets cookie. `409` if the email exists |
| POST   | `/api/auth/login`    | `{email, password}`                   | `200` user, sets cookie. `401` on bad credentials |
| POST   | `/api/auth/logout`   |                                       | `204`, clears cookie        |
| GET    | `/api/auth/me`       |                                       | `200` user, `401` if not signed in |
| GET    | `/health`            |                                       | `200` when the database answers, `503` otherwise |

User objects look like `{id, email, display_name, created_at}`. Unknown
`/api/*` paths return a JSON `404`; every other unknown path serves the app.

## Frontend

```sh
cd frontend
npm run dev        # Vite dev server (cargo run starts this for you by default)
npm run check      # svelte-check
npm run lint       # prettier + eslint
npm run build      # static bundle in build/, embedded by release builds
npx shadcn-svelte@latest add <component>   # add UI components
```

Routes:

| Path         | Purpose                                                        |
|--------------|----------------------------------------------------------------|
| `/`          | Signed-in home (redirects to `/login` otherwise)               |
| `/login`     | Login form                                                     |
| `/register`  | Registration form                                              |

The app is client-rendered (`ssr = false` in `src/routes/+layout.ts`), so
there are no server hooks, form actions or `+page.server.ts` files. The root
layout's `load` calls `/api/auth/me` and exposes the user as `data.user`; the
`(app)` and `(auth)` layout groups redirect based on it. Pages call the API
with `src/lib/api.ts` and navigate with `goto(..., { invalidateAll: true })`
after login, register and logout so the layouts re-run. The session cookie is
the backend's own; the browser sends it automatically on same-origin fetches.

Note: axum-vite marks only paths containing an `assets/` segment as
immutable. SvelteKit's hashed files live under `_app/immutable/`, so they are
served with `Cache-Control: public, no-cache` plus an `ETag`; browsers
revalidate and get `304`s rather than skipping the request.

## Layout

```
migration/          SeaORM migration crate (workspace member)
build.rs            rebuild when frontend/build changes; refuse release builds without it
src/main.rs         startup: config, db, migrations, Vite (debug), server
src/lib.rs          module tree + db connection
src/config.rs       environment configuration
src/state.rs        shared AppState
src/error.rs        AppError -> JSON error responses
src/extract.rs      JSON extractor with JSON-shaped rejections
src/frontend.rs     serves the web app: embedded in release, proxied to Vite in debug
src/auth/           password hashing, db-backed sessions, CurrentUser extractor
src/routes/         axum routers and handlers (API only)
src/entities/       generated SeaORM entities
tests/              integration tests against a real database
frontend/           SvelteKit app (see above)
Dockerfile          builds the web app, then the binary that embeds it
docker-compose.yml  db + app
```
