# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.82.0
ARG APP_NAME=databaseschema

################################################################################
# Stage 1: Build the application with optimizations
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME

# Install necessary build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libpq-dev \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Set performance-optimized environment variables
ENV RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C codegen-units=1 -C panic=abort"
ENV RUST_BACKTRACE=0

# Set the working directory inside the container
WORKDIR /app

# Copy the source code into the container
COPY . /app/databaseschema

COPY redisutils /app/redisutils

# Ensure the database schema builds correctly from the workspace
WORKDIR /app/databaseschema

RUN cargo test --locked --release && \
    cargo build --locked --release && \
    cp target/release/$APP_NAME /bin/server

################################################################################
# Stage 2: Create a smaller runtime image
FROM debian:bullseye-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libc6 \
    net-tools \
    procps \
    libssl-dev \
    ca-certificates \
    postgresql-client \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create the health check script
RUN echo '#!/bin/sh' > /usr/local/bin/health_check.sh \
&& echo 'if ! pgrep "server"; then exit 1; fi' >> /usr/local/bin/health_check.sh \
&& echo 'if ! pg_isready -h $POSTGRES_HOST -p $POSTGRES_PORT -U $POSTGRES_USERNAME -d $POSTGRES_DB; then exit 1; fi' >> /usr/local/bin/health_check.sh \
&& chmod +x /usr/local/bin/health_check.sh

# Create the liveness probe script
RUN echo '#!/bin/sh' > /usr/local/bin/liveness_check.sh \
&& echo 'if ! pgrep "server"; then exit 1; fi' >> /usr/local/bin/liveness_check.sh \
&& echo 'if ! pg_isready -h $POSTGRES_HOST -p $POSTGRES_PORT -U $POSTGRES_USERNAME -d $POSTGRES_DB; then exit 1; fi' >> /usr/local/bin/liveness_check.sh \
&& chmod +x /usr/local/bin/liveness_check.sh

# Create a non-privileged user to run the app
ARG UID=10001
RUN adduser --disabled-password --gecos "" --home "/nonexistent" --shell "/sbin/nologin" --no-create-home --uid "${UID}" appuser

# Copy the built application from the build stage
COPY --from=build /bin/server /bin/server

# Ensure the binary is executable
RUN chmod +x /bin/server

EXPOSE 443

# Switch to non-privileged user
USER appuser

# Set the command to run the application
CMD ["/bin/server"]