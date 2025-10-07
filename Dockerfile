# syntax=docker/dockerfile:1
# Purpose: Migration runner for Diesel database migrations
# Usage: Kubernetes Job to run `diesel migration run`

FROM rust:1.82.0-slim-bullseye

# Install diesel CLI and PostgreSQL client
RUN apt-get update && apt-get install -y \
    postgresql-client \
    libpq-dev \
    libssl-dev \
    pkg-config \
    && cargo install diesel_cli --no-default-features --features postgres \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only what's needed for migrations
COPY databaseschema/migrations /app/migrations
COPY databaseschema/diesel.toml /app/diesel.toml
COPY databaseschema/src /app/src
COPY databaseschema/Cargo.toml /app/Cargo.toml

# Default command runs migrations
# Can be overridden in Kubernetes Job
CMD ["diesel", "migration", "run"]