# syntax=docker/dockerfile:1
# Purpose: Migration runner for Diesel database migrations
# Usage: Kubernetes Job to run `diesel migration run`

FROM rust:1.82.0-slim-bookworm

# Install diesel CLI and PostgreSQL client
RUN apt-get update && apt-get install -y \
    postgresql-client \
    libpq-dev \
    libssl-dev \
    pkg-config \
    ca-certificates \
    && cargo install diesel_cli --no-default-features --features postgres \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

WORKDIR /app

# Copy only what's needed for migrations
COPY migrations ./migrations
COPY diesel.toml ./diesel.toml
COPY src ./src
COPY Cargo.toml ./Cargo.toml

# Set environment to ensure proper SSL
ENV PGSSLMODE=require

# Default command runs migrations
CMD ["diesel", "migration", "run"]