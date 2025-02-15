# Phonebook Application

This is a full-stack phonebook application originally built as part of Full Stack Open Part 3, now reimplemented in Rust.

## Tech Stack

- Frontend: Rust + Leptos (WASM)
- Backend: Rust + Axum
- Database: PostgreSQL
- Containerization: Docker

## Development Setup

### Prerequisites

#### General

- Docker installed

#### For local development
- Rust toolchain
- Trunk (for WASM development)
- SQLx-cli for running migrations on database

### Running in Development Mode

1. Clone the repository
2. Start the development environment:
```bash
docker compose -f compose.dev.yml up
```

The development environment includes:
- Frontend running on http://localhost (with hot reload via Trunk)
- Backend running on http://localhost/api
- PostgreSQL database

### Local Development (Without Docker)

1. Make sure you have database running with migrations

2. Start the backend:
```bash
cd phonebook-backend
DATABASE_URL=<url to database> cargo run
```

3. Start the frontend:
```bash
cd phonebook-frontend
trunk serve
```

## Production Build

To build and run the application in production mode:

1. Build the containers:
```bash
docker compose build
```

2. Start the application:
```bash
docker compose up
```

The production environment will be available at:
- Frontend: http://localhost
- Backend API: http://localhost/api
