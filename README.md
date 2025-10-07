# DatabaseSchema# Database Schema# DatabaseSchema# 🗃️ DatabaseSchema - Trading Platform Database Library



**PostgreSQL database library and migrations for the Trading Platform**



[![Rust](https://img.shields.io/badge/rust-latest-orange.svg)](https://www.rust-lang.org)**Production-ready PostgreSQL schema library for high-performance trading platforms**

[![PostgreSQL](https://img.shields.io/badge/postgresql-15+-blue.svg)](https://www.postgresql.org)

[![TimescaleDB](https://img.shields.io/badge/timescaledb-enabled-green.svg)](https://www.timescale.com)

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

## Overview**PostgreSQL database library and migrations for the Trading Platform.**

---



## What is this?

`databaseschema` is a dual-purpose Rust library that provides:

**This is a LIBRARY, not a standalone service.** It serves two purposes:

1. **Connection Pooling** - Async PostgreSQL connections with automatic retry logic

1. **Rust Library**: Imported by DataEngine and other services for PostgreSQL connection pooling

2. **Migration Runner**: Contains Diesel migrations that run in a Kubernetes Job2. **Migration Runner** - Diesel-based schema migrations for TimescaleDB + PostGIS[![Rust](https://img.shields.io/badge/rust-1.82+-orange.svg)](https://www.rust-lang.org)**PostgreSQL database library and migrations for the Trading Platform.**



### Important



- ❌ **NOT** a standalone microserviceThis library manages the complete database schema for a quantitative trading platform, including:[![PostgreSQL](https://img.shields.io/badge/postgresql-15+-blue.svg)](https://www.postgresql.org)

- ❌ **NO** HTTP server or API

- ✅ **Library** used by other services- Securities and exchange metadata

- ✅ **Contains** Diesel migrations for database schema

- Real-time order books and trades[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

---

- Simulation and backtesting data

## Table of Contents

- Historical snapshots and analytics

- [Usage as a Library](#usage-as-a-library)

- [Running Migrations](#running-migrations)- Strategy execution tracking

- [Database Schema](#database-schema)

- [Configuration](#configuration)---[![Rust](https://img.shields.io/badge/rust-1.82+-orange.svg)](https://www.rust-lang.org)**Rust library providing PostgreSQL schema and migrations for the Trading Platform.**[![Rust](https://img.shields.io/badge/rust-1.82+-orange.svg)](https://www.rust-lang.org)

- [CI/CD Automation](#cicd-automation)

- [Performance Optimization](#performance-optimization)## Architecture

- [Development](#development)

- [Troubleshooting](#troubleshooting)



---### Connection Pooling



## Usage as a Library## What is this?[![PostgreSQL](https://img.shields.io/badge/postgresql-15+-blue.svg)](https://www.postgresql.org)



### Add to Cargo.tomlUses **deadpool** for async connection pooling with intelligent retry logic:



```toml

[dependencies]

databaseschema = { path = "../databaseschema" }```rust

```

// Creates a pool of connections with exponential backoff retryThis is a **library**, not a standalone service. It has two purposes:[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)[![PostgreSQL](https://img.shields.io/badge/postgresql-15+-blue.svg)](https://www.postgresql.org)

### Create Connection Pool

create_timescale_connection_pool() -> Pool<AsyncPgConnection>

```rust

use databaseschema::{create_timescale_connection_pool, get_timescale_connection};

use std::sync::Arc;

// Gets a single connection with 3 retry attempts

#[tokio::main]

async fn main() {get_timescale_connection() -> Result<PooledConnection<AsyncPgConnection>, Error>1. **Rust Library** - Imported by DataEngine and other services for PostgreSQL connection pooling

    let pool = Arc::new(create_timescale_connection_pool());

    let conn = get_timescale_connection(pool).await.unwrap();```

    

    // Use connection for database operations2. **Migration Runner** - Diesel migrations that run in Kubernetes Jobs

}

```**Retry Strategy:**



### Connection Pooling Architecture- 3 attempts with exponential backoff---## What is this?[![TimescaleDB](https://img.shields.io/badge/timescaledb-enabled-green.svg)](https://www.timescale.com)



- **Deadpool**: Async connection pooling for production workloads- Delays: 2s → 4s → 8s

- **Retry Logic**: 3 attempts with exponential backoff (2s → 4s → 8s)

- **Auto-reconnect**: Handles transient failures automatically- Automatic reconnection on transient failures**Important:**



---



## Running Migrations**Configuration** (via environment variables):- ❌ NOT a standalone microservice



### Local Development```bash



```bashDATABASE_URL=postgresql://user:pass@host:port/dbname- ❌ NO HTTP server or API

# Install diesel CLI

cargo install diesel_cli --no-default-features --features postgresPOOL_SIZE=10              # Optional, default: 10



# Run migrations```- ✅ Library used by other services## What is this?[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

diesel migration run



# Rollback last migration

diesel migration revert### Dependencies- ✅ Contains Diesel migrations

```



### Production (Kubernetes)

| Dependency | Version | Purpose |

Migrations run automatically via a Kubernetes Job using the `databaseschema` Docker image:

|------------|---------|---------|

```yaml

# Deployed via k8s/migrations.yaml| `diesel` | 2.2.2 | SQL query builder with PostgreSQL support |---

apiVersion: batch/v1

kind: Job| `diesel-async` | 0.5.0 | Async operations with deadpool integration |

metadata:

  name: database-migrations| `deadpool` | - | Connection pooling for production workloads |This is a **library**, not a standalone service. It has two purposes:This is a **library**, not a standalone service. It serves two purposes:

spec:

  template:| `bigdecimal` | - | High-precision financial calculations |

    spec:

      containers:| `chrono` | - | Timezone-aware timestamp handling |## Usage as a Library

      - name: migrations

        image: ghcr.io/nwagbara-group-llc/databaseschema:latest| `tokio-retry` | - | Exponential backoff retry logic |

        env:

        - name: DATABASE_URL| `native-tls` | - | SSL/TLS for secure database connections |

          value: "postgres://user:pass@postgresql:5432/dbname"

        # Default CMD is: diesel migration run

```

**Diesel Features Enabled:**```toml

The Job executes `diesel migration run` against your PostgreSQL instance.

- `postgres` - PostgreSQL database support

---

- `uuid-ossp` - UUID generation# Cargo.toml1. **Rust Library** - Imported by DataEngine and other services for PostgreSQL connection poolingA **Rust library** providing database schema, connection pooling, and Diesel migrations for the Trading Platform ecosystem.

## Database Schema

- `numeric` - `bigdecimal` support for precise decimals

The library manages 18 migrations covering the complete trading platform schema:

- `64-column-tables` - Support for wide tables[dependencies]

### Core Trading Infrastructure (2024-12-27)



| Migration | Purpose |

|-----------|---------|## Database Schemadatabaseschema = { path = "../databaseschema" }2. **Migration Runner** - Diesel migrations that run in Kubernetes Jobs

| `create_securities` | Security master data (symbols, exchanges, asset types) |

| `create_exchanges` | Exchange metadata and trading hours |

| `create_order_books` | Real-time order book data (TimescaleDB hypertable) |

| `create_buy_orders` | Live buy order flow (LIMIT, MARKET, STOP) |The schema consists of **18 migrations** organized chronologically:```

| `create_sell_orders` | Live sell order flow with status tracking |

| `create_trades` | Executed trade records with buyer/seller matching |



### Simulation & Testing (2025-02-08, 2025-02-14, 2025-02-26)### Core Trading Infrastructure (2024-12-27)1. **Rust Library**: Imported by DataEngine and other services for PostgreSQL connection pooling



| Migration | Purpose |

|-----------|---------|

| `create_sim_buy_orders` | Paper trading buy orders (isolated environment) |**1. Securities** - Master security definitions```rust

| `create_sim_sell_orders` | Paper trading sell orders |

| `create_sim_trades` | Simulated executions for backtesting |- `security_id` (UUID, primary key)

| `create_historical_orders` | Order book snapshots for market replay |

| `create_historical_snapshot` | Aggregated OHLCV historical data |- `symbol`, `exchange`, `security_type`use databaseschema::{create_timescale_connection_pool, get_timescale_connection};**Important:**



### Backtesting Framework (2025-08-17, 2025-08-23)- `tick_size`, `lot_size` for precision



| Migration | Purpose |use std::sync::Arc;

|-----------|---------|

| `create_backtest_results` | Performance metrics (P&L, Sharpe, drawdown) |**2. Exchanges** - Exchange metadata

| `create_strategy_schema` | Strategy definitions with JSONB parameters |

| `create_backtest_related_tables` | Position history and trade breakdown |- `exchange_id`, `exchange_name`, `exchange_code`- ❌ NOT a standalone microservice2. **Migration Runner**: Diesel migrations run in a Kubernetes Job---

| `create_backtest_reports` | HTML/JSON tear sheets and analytics |

- Trading hours and timezone support

### Market Data & Strategy Execution (2025-08-24, 2025-08-28)

#[tokio::main]

| Migration | Purpose |

|-----------|---------|**3. Order Books** - Real-time market data

| `create_candles` | OHLCV bars (1m, 5m, 1h, 1d) with continuous aggregates |

| `create_strategy_orders` | Algorithm order tracking and performance attribution |- `order_book_id`, `security_id`, `timestamp`async fn main() {- ❌ NO HTTP server or API



### Key Features- Best bid/ask tracking



- **TimescaleDB Hypertables**: Automatic partitioning for time-series data    let pool = Arc::new(create_timescale_connection_pool());

- **Foreign Key Constraints**: Data integrity across tables

- **Indexes**: Optimized for high-frequency trading queries**4. Buy Orders & 5. Sell Orders** - Live order flow

- **JSONB**: Flexible strategy parameters storage

- `order_id`, `security_id`, `price`, `quantity`    let conn = get_timescale_connection(pool).await.unwrap();- ✅ Library used by other services

---

- `order_type` (LIMIT, MARKET, STOP)

## Configuration

- `status` (PENDING, FILLED, CANCELLED)    // Use connection for database operations

### Environment Variables

- TimescaleDB hypertable for time-series optimization

Create a `.env` file (see `.env.example`):

}- ✅ Contains Diesel migrations

```bash

DATABASE_URL=postgres://user:password@host:5432/database?sslmode=require**6. Trades** - Executed trade records

```

- `trade_id`, `security_id`, `price`, `quantity````

**Never commit the `.env` file!** It contains credentials.

- `buyer_order_id`, `seller_order_id`

### PostgreSQL Requirements

- `timestamp` with TimescaleDB indexing## Usage as a Library## 🎯 **What is this?**

- **PostgreSQL 15+** with PostGIS extension

- **TimescaleDB** for time-series optimization

- **Extensions**: uuid-ossp, timescaledb, postgis

### Simulation & Testing (2025-02-08, 2025-02-14, 2025-02-26)---

---



## CI/CD Automation

**7. Sim Buy Orders & 8. Sim Sell Orders** - Paper trading---

### GitHub Actions Workflow

- Isolated simulation environment

**Automatic builds triggered by:**

- Changes to `migrations/**`- Same schema as live orders## Running Migrations

- Updates to `src/**`

- Modifications to `Dockerfile`



**Workflow process:****9. Sim Trades** - Simulated executions

1. Checkout code

2. Login to GitHub Container Registry (GHCR)- Backtesting and strategy validation

3. Build Docker image (migration runner)

4. Push to `ghcr.io/nwagbara-group-llc/databaseschema:latest`### Local Development

5. Tag with Git SHA for traceability

**10. Historical Orders** - Order book snapshots

### Image Tags

- Point-in-time reconstruction## Usage as a Library

```bash

ghcr.io/nwagbara-group-llc/databaseschema:latest       # Most recent build- Market replay capabilities

ghcr.io/nwagbara-group-llc/databaseschema:sha-abc123   # Specific commit

``````bash



**No manual steps needed!** Just `git push`.**11. Historical Snapshot** - Aggregated market data



---- OHLCV (Open, High, Low, Close, Volume)# Install diesel CLI```toml**DatabaseSchema is a LIBRARY, not a standalone service.**



## Performance Optimization- Time-series compression



### TimescaleDB Hypertablescargo install diesel_cli --no-default-features --features postgres



```sql### Backtesting Framework (2025-08-17, 2025-08-23)

-- Create hypertable for time-series data

SELECT create_hypertable('order_books', 'timestamp', chunk_time_interval => INTERVAL '1 day');```toml



-- Enable compression (reduce storage by 90%+)**12. Backtest Results** - Performance metrics

ALTER TABLE order_books SET (

  timescaledb.compress,- `backtest_id`, `strategy_id`, `start_date`, `end_date`# Run migrations

  timescaledb.compress_segmentby = 'security_id'

);- P&L, Sharpe ratio, max drawdown



-- Automated compression policy- Execution statisticsdiesel migration run# Cargo.toml# Cargo.toml

SELECT add_compression_policy('order_books', INTERVAL '7 days');

```



### Continuous Aggregates**13. Strategy Schema** - Strategy definitions



```sql- `strategy_id`, `name`, `parameters` (JSONB)

-- Pre-computed rollups for candles

CREATE MATERIALIZED VIEW candles_1h- Version tracking# Rollback last migration[dependencies]

WITH (timescaledb.continuous) AS

SELECT time_bucket('1 hour', timestamp) AS bucket,

       security_id,

       first(price, timestamp) AS open,**14. Backtest Related Tables** - Execution detailsdiesel migration revert

       max(price) AS high,

       min(price) AS low,- Position history

       last(price, timestamp) AS close,

       sum(quantity) AS volume- Trade-by-trade breakdown```databaseschema = { path = "../databaseschema" }[dependencies]It serves two purposes:

FROM trades

GROUP BY bucket, security_id;- Risk metrics

```



### Indexing Strategy

**15. Backtest Reports** - HTML/JSON reports

```sql

-- Essential indexes for trading queries- Tear sheets and analytics### Production (Kubernetes)```

CREATE INDEX idx_trades_timestamp ON trades (timestamp DESC);

CREATE INDEX idx_trades_security ON trades (security_id, timestamp DESC);- Custom report generation

CREATE INDEX idx_orders_status ON buy_orders (status, price DESC);

```



---### Market Data (2025-08-24)



## DevelopmentMigrations run automatically via GitHub Actions and Kubernetes Jobs.databaseschema = { path = "../databaseschema" }1. **Rust Library**: Imported by DataEngine and other services for PostgreSQL connection pooling



### Creating New Migrations**16. Candles** - OHLCV bars



```bash- Multiple timeframes (1m, 5m, 1h, 1d)

# Generate migration files

diesel migration generate add_new_table- TimescaleDB continuous aggregates



# Edit the generated files- Efficient range queriesThe workflow builds a Docker image that runs `diesel migration run`:```rust

# migrations/YYYY-MM-DD-HHMMSS_add_new_table/up.sql

# migrations/YYYY-MM-DD-HHMMSS_add_new_table/down.sql



# Test locally### Strategy Execution (2025-08-28)

diesel migration run

diesel migration revert

diesel migration run

**17. Strategy Orders** - Algorithm order tracking```yamluse databaseschema::{create_timescale_connection_pool, get_timescale_connection};```2. **Migration Runner**: Contains Diesel migrations that run in a Kubernetes Job

# Commit and push

git add migrations/- Links strategies to executed orders

git commit -m "Add new migration"

git push  # GitHub Actions builds and pushes automatically- Performance attribution# Deployed via k8s/migrations.yaml

```

- Slippage analysis

### Local Setup

apiVersion: batch/v1use std::sync::Arc;

```bash

# Install diesel CLI## Usage

cargo install diesel_cli --no-default-features --features postgres

kind: Job

# Set up PostgreSQL with TimescaleDB

docker run -d \### As a Library

  -p 5432:5432 \

  -e POSTGRES_PASSWORD=password \metadata:

  timescale/timescaledb:latest-pg15

Add to your `Cargo.toml`:

# Create .env file

cp .env.example .env```toml  name: database-migrations

# Edit .env with your DATABASE_URL

[dependencies]

# Run migrations

diesel migration rundatabaseschema = { path = "../databaseschema" }spec:#[tokio::main]



# Build library```

cargo build

  template:

# Run tests

cargo test**Create a connection pool:**

```

```rust    spec:async fn main() {```rust### **Important**

---

use databaseschema::create_timescale_connection_pool;

## Troubleshooting

      containers:

### Connection Issues

#[tokio::main]

**Problem:** `Failed to get connection after retries`

async fn main() {      - name: migrations    let pool = Arc::new(create_timescale_connection_pool());

- **Check**: DATABASE_URL is correct

- **Verify**: PostgreSQL is running and accessible    dotenv::dotenv().ok();

- **Test**: `psql $DATABASE_URL`

            image: ghcr.io/nwagbara-group-llc/databaseschema:latest

### Migration Failures

    let pool = create_timescale_connection_pool();

**Problem:** `diesel migration run` fails

    let mut conn = pool.get().await.expect("Failed to get connection");        env:    let conn = get_timescale_connection(pool).await.unwrap();use databaseschema::{create_timescale_connection_pool, get_timescale_connection};- ❌ **NOT** a standalone microservice

- **Check**: PostgreSQL version (15+ required)

- **Verify**: TimescaleDB extension installed    

- **Review**: Migration logs in `diesel_migrations` table

- **Rollback**: `diesel migration revert` and fix SQL    // Use conn with diesel-async queries        - name: DATABASE_URL



### Docker Build Failures}



**Problem:** Rust version mismatch```          valueFrom:    // Use connection for database operations



- **Solution**: Dockerfile uses `rust:latest-slim-bookworm`

- **Pin**: diesel_cli to version 2.2.4 for stability

**With retry logic:**            secretKeyRef:

**Problem:** GHCR authentication failure

```rust

- **Check**: `PERSONAL_ACCESS_TOKEN` has `read:packages` scope

- **Verify**: Token is active and not expireduse databaseschema::get_timescale_connection;              name: database-secret}use std::sync::Arc;- ❌ **NO** HTTP server or API



---



## Dependencieslet mut conn = get_timescale_connection()              key: url



| Dependency | Version | Purpose |    .await

|------------|---------|---------|

| `diesel` | 2.2.2 | SQL query builder with PostgreSQL support |    .expect("Failed to get connection after retries");``````

| `diesel-async` | 0.5.0 | Async operations with deadpool integration |

| `deadpool` | - | Connection pooling for production workloads |```

| `bigdecimal` | - | High-precision financial calculations |

| `chrono` | - | Timezone-aware timestamp handling |

| `tokio-retry` | - | Exponential backoff retry logic |

| `native-tls` | - | SSL/TLS for secure database connections |### Running Migrations



**Diesel Features Enabled:**---- ✅ **Library** used by other services

- `postgres` - PostgreSQL database support

- `uuid-ossp` - UUID generation**Prerequisites:**

- `numeric` - BigDecimal support for precise decimals

- `64-column-tables` - Support for wide tables```bash



---# Install Diesel CLI (version 2.2.4 recommended)



## Used Bycargo install diesel_cli --version 2.2.4 --no-default-features --features postgres## Configuration---



- **DataEngine**: Market data ingestion and real-time price storage

- **SignalEngine**: Signal generation state and portfolio position tracking

- **BacktestingEngine**: Historical data access and backtest result storage# Set database URL



All services import this library for database connectivity instead of implementing their own connection management.export DATABASE_URL=postgresql://user:pass@localhost:5432/tradingdb



---```Create a `.env` file (see `.env.example`):#[tokio::main]- ✅ **Contains** Diesel migrations for database schema



## License



Apache 2.0**Apply all migrations:**



---```bash



**Built with:** Rust 🦀 | Diesel | TimescaleDB | PostgreSQLdiesel migration run```bash## Running Migrations


```

DATABASE_URL=postgres://user:password@host:5432/database?sslmode=require

**Rollback last migration:**

```bash```async fn main() {

diesel migration revert

```



**Check migration status:****Never commit the `.env` file!** It contains credentials.### Local Development

```bash

diesel migration list

```

---    let pool = Arc::new(create_timescale_connection_pool());---

### Creating New Migrations



```bash

# Generate migration files## Database Schema```bash

diesel migration generate <migration_name>



# Edit the generated up.sql and down.sql files

# Example: migrations/2025-XX-XX-XXXXXX_<migration_name>/up.sqlThe library provides:# Install diesel CLI    let conn = get_timescale_connection(pool).await.unwrap();



# Test migration

diesel migration run

diesel migration revert- **TimescaleDB hypertables** for time-series market datacargo install diesel_cli --no-default-features --features postgres

diesel migration run

```- **PostgreSQL with PostGIS** for geospatial data



**Best Practices:**- **Diesel ORM** for type-safe queries    // Use connection...## � **Usage**

- Name migrations descriptively: `create_order_execution_table`

- Always write reversible `down.sql` migrations- **Deadpool** for async connection pooling

- Test rollback before committing

- Use TimescaleDB for time-series tables:# Run migrations

  ```sql

  CREATE TABLE trades (...);### Core Tables

  SELECT create_hypertable('trades', 'timestamp');

  ```diesel migration run}



## CI/CD Automation- `securities` - Security master data (symbols, exchanges)



### GitHub Actions Workflow- `order_books` - Real-time order book data (TimescaleDB hypertable)



**Trigger:** Automatic builds on changes to:- `trades` - Trade execution history

- `migrations/**`

- `src/**`- `positions` - Portfolio positions# Rollback last migration```### **As a Library (in other Rust projects)**

- `Dockerfile`

- `backtest_results` - Backtesting results

**Process:**

1. Checkout codediesel migration revert

2. Login to GitHub Container Registry (GHCR)

3. Build Docker image (migration runner)---

4. Push to `ghcr.io/nwagbara-group-llc/databaseschema:latest`

5. Tag with Git SHA for traceability```



**Image Tags:**## CI/CD

- `latest` - Most recent build

- `sha-<git-sha>` - Specific commit version



### Docker Image### Automatic Image Builds



**Purpose:** Kubernetes Job for applying migrations### Production (Kubernetes)## Running Migrations```rust



**Usage in Kubernetes:**GitHub Actions automatically builds and pushes to GHCR when you:

```yaml

apiVersion: batch/v1- Change migrations

kind: Job

metadata:- Update source code

  name: database-migrations

spec:- Modify DockerfileMigrations run automatically via GitHub Actions and Kubernetes Jobs.use databaseschema::{create_timescale_connection_pool, get_timescale_connection};

  template:

    spec:

      containers:

      - name: migrations**No manual steps needed!** Just `git push`.

        image: ghcr.io/nwagbara-group-llc/databaseschema:latest

        env:

        - name: DATABASE_URL

          valueFrom:### Image LocationThe workflow builds a Docker image that runs `diesel migration run`:### Local Developmentuse std::sync::Arc;

            secretKeyRef:

              name: db-credentials

              key: url

      restartPolicy: OnFailure```

```

ghcr.io/nwagbara-group-llc/databaseschema:latest

**Image Contents:**

- Diesel CLI 2.2.4ghcr.io/nwagbara-group-llc/databaseschema:sha-abc123```yaml

- PostgreSQL client tools

- All migration files```

- SSL/TLS support

# Deployed via k8s/migrations.yaml

## Configuration

---

### Environment Variables

apiVersion: batch/v1```bash#[tokio::main]

| Variable | Required | Default | Description |

|----------|----------|---------|-------------|## Creating Migrations

| `DATABASE_URL` | Yes | - | PostgreSQL connection string |

| `POOL_SIZE` | No | 10 | Connection pool size |kind: Job

| `PGSSLMODE` | No | `prefer` | SSL mode (require, prefer, disable) |

```bash

**Example `.env`:**

```bash# Create new migrationmetadata:# Install diesel CLIasync fn main() {

DATABASE_URL=postgresql://trading_user:secure_password@postgres:5432/trading_db?sslmode=require

POOL_SIZE=20diesel migration generate add_new_table

PGSSLMODE=require

```  name: database-migrations



### PostgreSQL Requirements# Edit the generated files



**Minimum Version:** PostgreSQL 15+# migrations/YYYY-MM-DD-HHMMSS_add_new_table/up.sqlspec:cargo install diesel_cli --no-default-features --features postgres    // Create connection pool



**Required Extensions:**# migrations/YYYY-MM-DD-HHMMSS_add_new_table/down.sql

```sql

CREATE EXTENSION IF NOT EXISTS timescaledb;  template:

CREATE EXTENSION IF NOT EXISTS postgis;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";# Test locally

```

diesel migration run    spec:    let pool = Arc::new(create_timescale_connection_pool());

**Recommended Configuration:**

```sqldiesel migration revert

-- For time-series optimization

ALTER DATABASE trading_db SET timescaledb.max_background_workers = 8;diesel migration run      containers:



-- For large result sets

ALTER DATABASE trading_db SET work_mem = '256MB';

```# Commit and push      - name: migrations# Run migrations    



## Performance Optimizationgit add migrations/



### TimescaleDB Featuresgit commit -m "Add new migration"        image: ghcr.io/nwagbara-group-llc/databaseschema:latest



**Hypertables** - Automatic partitioning for time-series data:git push  # GitHub Actions builds and pushes automatically

```sql

SELECT create_hypertable('trades', 'timestamp', chunk_time_interval => INTERVAL '1 day');```        env:diesel migration run    // Get connection from pool

```



**Continuous Aggregates** - Pre-computed rollups:

```sql---        - name: DATABASE_URL

CREATE MATERIALIZED VIEW candles_1h

WITH (timescaledb.continuous) AS

SELECT time_bucket('1 hour', timestamp) AS bucket,

       security_id,## Dependencies          valueFrom:```    let conn = get_timescale_connection(pool).await.unwrap();

       first(price, timestamp) AS open,

       max(price) AS high,

       min(price) AS low,

       last(price, timestamp) AS close,- **PostgreSQL 15+** with PostGIS extension            secretKeyRef:

       sum(quantity) AS volume

FROM trades- **TimescaleDB** for time-series optimization

GROUP BY bucket, security_id;

```- **Diesel ORM** for migrations and queries              name: database-secret    



**Compression** - Reduce storage by 90%+:- **Deadpool** for async connection pooling

```sql

ALTER TABLE trades SET (              key: url

  timescaledb.compress,

  timescaledb.compress_orderby = 'timestamp DESC',---

  timescaledb.compress_segmentby = 'security_id'

);```### Kubernetes (Production)    // Use connection...



SELECT add_compression_policy('trades', INTERVAL '7 days');## Used By

```



### Indexing Strategy

- **DataEngine** - Market data processing

**Essential Indexes:**

```sql- **SignalEngine** - Trading signal generation---}

-- Time-range queries

CREATE INDEX idx_trades_timestamp ON trades (timestamp DESC);- **BacktestingEngine** - Historical backtesting



-- Security lookups

CREATE INDEX idx_trades_security ON trades (security_id, timestamp DESC);

All services import this library for database connectivity instead of implementing their own connection management.

-- Order matching

CREATE INDEX idx_orders_status ON buy_orders (status, price DESC);## ConfigurationMigrations run automatically via a Kubernetes Job using the `databaseschema` Docker image:```

```

---

## Troubleshooting



### Connection Issues

## License

**Problem:** `Failed to get connection after retries`

- **Check:** DATABASE_URL is correctCreate a `.env` file (see `.env.example`):

- **Verify:** PostgreSQL is running and accessible

- **Test:** `psql $DATABASE_URL`Apache 2.0

- **Increase:** Retry attempts in `get_timescale_connection()`



### Migration Failures

```bash```bash### **Running Migrations (Kubernetes Job)**

**Problem:** `diesel migration run` fails

- **Check:** PostgreSQL version (15+ required)DATABASE_URL=postgres://user:password@host:5432/database?sslmode=require

- **Verify:** TimescaleDB extension installed

- **Review:** Migration logs in `diesel_migrations` table```kubectl apply -f k8s/migrations.yaml

- **Rollback:** `diesel migration revert` and fix SQL



### Docker Build Failures

**Never commit the `.env` file!** It contains credentials.```The Docker image is used ONLY for running migrations:

**Problem:** Rust version mismatch

- **Solution:** Use `rust:latest-slim-bookworm` base image

- **Pin:** diesel_cli to 2.2.4 for stability

---

**Problem:** GHCR authentication failure

- **Check:** `PERSONAL_ACCESS_TOKEN` has `read:packages` scope

- **Verify:** Token is active and not expired

## Database SchemaThe Job executes `diesel migration run` against your PostgreSQL instance.```yaml

## Development



### Local Setup

The library provides:# Kubernetes Job

```bash

# 1. Clone repository

git clone https://github.com/nwagbara-group-llc/databaseschema.git

cd databaseschema- **TimescaleDB hypertables** for time-series market data## ConfigurationapiVersion: batch/v1



# 2. Install dependencies- **PostgreSQL with PostGIS** for geospatial data

cargo build

- **Diesel ORM** for type-safe querieskind: Job

# 3. Set up PostgreSQL with TimescaleDB

docker run -d \- **Deadpool** for async connection pooling

  -p 5432:5432 \

  -e POSTGRES_PASSWORD=password \Set `DATABASE_URL` in your environment:metadata:

  timescale/timescaledb:latest-pg15

### Core Tables

# 4. Create .env file

cp .env.example .env  name: database-migrations

# Edit .env with your DATABASE_URL

- `securities` - Security master data (symbols, exchanges)

# 5. Run migrations

diesel migration run- `order_books` - Real-time order book data (TimescaleDB hypertable)```bashspec:



# 6. Run tests- `trades` - Trade execution history

cargo test

```- `positions` - Portfolio positions# .env file (NEVER commit this!)  template:



### Testing- `backtest_results` - Backtesting results



```bashDATABASE_URL=postgres://user:password@host:5432/database?sslmode=require    spec:

# Unit tests

cargo test---



# Integration tests (requires running PostgreSQL)```      containers:

cargo test --features integration-tests

## CI/CD

# Check migrations

diesel migration list      - name: migrations

```

### Automatic Image Builds

## Security

## Dependencies        image: ghcr.io/nwagbara-group-llc/databaseschema:latest

- **Never commit `.env` files** - Use `.env.example` templates

- **Use SSL/TLS** - Set `PGSSLMODE=require` in productionGitHub Actions automatically builds and pushes to GHCR when you:

- **Rotate credentials** - Update `DATABASE_URL` regularly

- **Limit permissions** - Database user should have minimum required privileges- Change migrations        env:



## License- Update source code



See [LICENSE](LICENSE) file for details.- Modify Dockerfile- PostgreSQL 15+ with PostGIS extension        - name: DATABASE_URL



## Support



For issues or questions:**No manual steps needed!** Just `git push`.- TimescaleDB (for time-series data)          value: "postgres://user:pass@postgresql:5432/dbname"

1. Check [Troubleshooting](#troubleshooting) section

2. Review [GitHub Issues](https://github.com/nwagbara-group-llc/databaseschema/issues)

3. Contact: [Your Contact Info]

### Image Location- Diesel ORM for migrations and query builder        # Default CMD is: diesel migration run

---



**Built with:** Rust 🦀 | Diesel | TimescaleDB | PostgreSQL

```- Deadpool for async connection pooling```

ghcr.io/nwagbara-group-llc/databaseschema:latest

ghcr.io/nwagbara-group-llc/databaseschema:sha-abc123

```

## Used By---

---



## Creating Migrations

- **DataEngine** - Market data processing## 🗄️ **Migrations**

```bash

# Create new migration- **SignalEngine** - Trading signal generation

diesel migration generate add_new_table

- **BacktestingEngine** - Historical backtesting### **Running Migrations Locally**

# Edit the generated files

# migrations/YYYY-MM-DD-HHMMSS_add_new_table/up.sql- **Data Integrity**: Foreign key constraints, triggers, and data validation

# migrations/YYYY-MM-DD-HHMMSS_add_new_table/down.sql

## License- **Backup Integration**: Automated backup scheduling and point-in-time recovery

# Test locally

diesel migration run- **Kubernetes Ready**: Helm charts for scalable cloud deployment

diesel migration revert

diesel migration runApache 2.0



# Commit and push---

git add migrations/

git commit -m "Add new migration"## 🏗️ **Architecture**

git push  # GitHub Actions builds and pushes automatically

```### **Database Layer Architecture**

```

---┌─────────────────────────────────────────────────────────────┐

│                DatabaseSchema Core                          │

## Dependencies├─────────────────────────────────────────────────────────────┤

│                                                             │

- **PostgreSQL 15+** with PostGIS extension│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │

- **TimescaleDB** for time-series optimization│  │ Connection  │    │  Migration  │    │TimescaleDB  │     │

- **Diesel ORM** for migrations and queries│  │ Pool Mgmt   │◄──►│  Manager    │◄──►│ Extensions  │     │

- **Deadpool** for async connection pooling│  │             │    │             │    │             │     │

│  │ • Deadpool  │    │ • Diesel    │    │ • Hypertables│     │

---│  │ • Health    │    │ • Versioning│    │ • Compression│     │

│  │ • Retry     │    │ • Rollback  │    │ • Partitions│     │

## Used By│  └─────────────┘    └─────────────┘    └─────────────┘     │

│         │                    │                    │         │

- **DataEngine** - Market data processing│         ▼                    ▼                    ▼         │

- **SignalEngine** - Trading signal generation│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │

- **BacktestingEngine** - Historical backtesting│  │Schema       │    │Data Models  │    │Query        │     │

│  │Definition   │    │& Relations  │    │Optimization │     │

All services import this library for database connectivity instead of implementing their own connection management.│  │             │    │             │    │             │     │

│  │ • Tables    │    │ • Structs   │    │ • Indexing  │     │

---│  │ • Indexes   │    │ • Traits    │    │ • Caching   │     │

│  │ • Triggers  │    │ • Validation│    │ • Prepared  │     │

## License│  └─────────────┘    └─────────────┘    └─────────────┘     │

└─────────────────────────────────────────────────────────────┘

Apache 2.0```


### **Integration with Trading Services**
- **DataEngine**: Market data ingestion and real-time price storage
- **SignalEngine**: Signal generation state and portfolio position tracking
- **BacktestingEngine**: Historical data access and backtest result storage
- **MessageBrokerEngine**: Event sourcing and message persistence

---

## 📊 **Database Schema**

### **Core Trading Tables**
```sql
-- Securities Master Data
CREATE TABLE securities (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL UNIQUE,
    exchange VARCHAR(10) NOT NULL,
    asset_type VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Order Book Data (TimescaleDB Hypertable)
CREATE TABLE order_books (
    timestamp TIMESTAMPTZ NOT NULL,
    security_id INTEGER REFERENCES securities(id),
    bid_price DECIMAL(20,8) NOT NULL,
    ask_price DECIMAL(20,8) NOT NULL,
    bid_size DECIMAL(20,8) NOT NULL,
    ask_size DECIMAL(20,8) NOT NULL
);

SELECT create_hypertable('order_books', 'timestamp');

-- Trade Execution History
CREATE TABLE trades (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL,
    security_id INTEGER REFERENCES securities(id),
    price DECIMAL(20,8) NOT NULL,
    quantity DECIMAL(20,8) NOT NULL,
    side VARCHAR(4) NOT NULL CHECK (side IN ('BUY', 'SELL')),
    execution_id UUID NOT NULL UNIQUE
);

-- Portfolio Positions
CREATE TABLE positions (
    id SERIAL PRIMARY KEY,
    account_id VARCHAR(50) NOT NULL,
    security_id INTEGER REFERENCES securities(id),
    quantity DECIMAL(20,8) NOT NULL,
    average_price DECIMAL(20,8) NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(account_id, security_id)
);
```

### **Migration History**
| Migration | Description | Date |
|-----------|-------------|------|
| `2024-12-27-005549_create_securities` | Securities master table | 2024-12-27 |
| `2024-12-27-005559_create_exchanges` | Exchange configuration | 2024-12-27 |
| `2024-12-27-005607_create_order_books` | Order book data storage | 2024-12-27 |
| `2025-02-08-122653_create_sim_buy_orders` | Simulation buy orders | 2025-02-08 |
| `2025-08-17-000001_create_backtest_results` | Backtest result storage | 2025-08-17 |

---

## ⚡ **Quick Start**

### **Installation**
Add to your `Cargo.toml`:
```toml
[dependencies]
databaseschema = { path = "../databaseschema" }
diesel = { version = "2.0", features = ["postgres", "chrono"] }
diesel-async = { version = "0.4", features = ["postgres", "deadpool"] }
tokio = { version = "1.0", features = ["full"] }
```

### **Basic Usage**
```rust
use databaseschema::establish_connection;
use diesel_async::RunQueryDsl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create database connection pool
    let pool = establish_connection().await?;
    
    // Get connection from pool
    let mut conn = pool.get().await?;
    
    // Query securities
    use databaseschema::schema::securities::dsl::*;
    let results = securities
        .filter(symbol.eq("BTCUSD"))
        .load::<Security>(&mut conn)
        .await?;
        
    println!("Found {} securities", results.len());
    Ok(())
}
```

---

## 🔧 **Configuration**

### **Environment Variables**
Create a `.env` file:
```env
# Database Configuration
DATABASE_URL=postgresql://username:password@localhost:5432/trading_platform
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_USER=trading_platform
POSTGRES_PASSWORD=secure_password
POSTGRES_DB=trading_platform

# Connection Pool Settings
DATABASE_POOL_MAX_SIZE=100
DATABASE_CONNECTION_TIMEOUT=30
DATABASE_IDLE_TIMEOUT=600
DATABASE_MAX_LIFETIME=3600

# TLS Configuration
DATABASE_TLS_ENABLED=true
DATABASE_TLS_CERT_PATH=/path/to/client.crt
DATABASE_TLS_KEY_PATH=/path/to/client.key
DATABASE_TLS_CA_PATH=/path/to/ca.crt

# TimescaleDB Configuration
TIMESCALEDB_ENABLED=true
TIMESCALEDB_COMPRESSION=true
TIMESCALEDB_CHUNK_TIME_INTERVAL=1h
```

### **Production Settings**
| Parameter | Development | Production | High-Frequency |
|-----------|-------------|------------|----------------|
| `DATABASE_POOL_MAX_SIZE` | 10 | 100 | 500 |
| `DATABASE_CONNECTION_TIMEOUT` | 30s | 10s | 5s |
| `DATABASE_IDLE_TIMEOUT` | 600s | 300s | 120s |
| `TIMESCALEDB_CHUNK_TIME_INTERVAL` | 1h | 15m | 1m |

---

## 🗄️ **Migrations**

### **Running Migrations**
```bash
# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Setup database
diesel setup

# Run all pending migrations
diesel migration run

# Rollback last migration
diesel migration revert

# Check migration status
diesel migration list
```

### **Creating New Migrations**
```bash
# Create new migration
diesel migration generate create_new_table

# Edit migration files
# migrations/YYYY-MM-DD-HHMMSS_create_new_table/up.sql
# migrations/YYYY-MM-DD-HHMMSS_create_new_table/down.sql

# Test migration
diesel migration run
diesel migration revert
diesel migration run
```

---

## 📦 **Usage Examples**

### **Market Data Storage**
```rust
use databaseschema::{models::OrderBook, establish_connection};
use diesel_async::RunQueryDsl;
use chrono::Utc;

async fn store_market_data() -> Result<(), Box<dyn std::error::Error>> {
    let pool = establish_connection().await?;
    let mut conn = pool.get().await?;
    
    let order_book = OrderBook {
        timestamp: Utc::now().naive_utc(),
        security_id: 1,
        bid_price: BigDecimal::from_str("50000.00")?,
        ask_price: BigDecimal::from_str("50001.00")?,
        bid_size: BigDecimal::from_str("1.5")?,
        ask_size: BigDecimal::from_str("2.0")?,
    };
    
    diesel::insert_into(order_books::table)
        .values(&order_book)
        .execute(&mut conn)
        .await?;
        
    Ok(())
}
```

### **Trade Execution Tracking**
```rust
async fn record_trade() -> Result<(), Box<dyn std::error::Error>> {
    let pool = establish_connection().await?;
    let mut conn = pool.get().await?;
    
    let trade = Trade {
        timestamp: Utc::now().naive_utc(),
        security_id: 1,
        price: BigDecimal::from_str("50000.50")?,
        quantity: BigDecimal::from_str("1.0")?,
        side: "BUY".to_string(),
        execution_id: Uuid::new_v4(),
    };
    
    diesel::insert_into(trades::table)
        .values(&trade)
        .execute(&mut conn)
        .await?;
        
    Ok(())
}
```

### **Portfolio Position Updates**
```rust
async fn update_position() -> Result<(), Box<dyn std::error::Error>> {
    let pool = establish_connection().await?;
    let mut conn = pool.get().await?;
    
    diesel::insert_into(positions::table)
        .values(&Position {
            account_id: "trader123".to_string(),
            security_id: 1,
            quantity: BigDecimal::from_str("10.5")?,
            average_price: BigDecimal::from_str("49850.25")?,
            updated_at: Utc::now().naive_utc(),
        })
        .on_conflict((account_id, security_id))
        .do_update()
        .set((
            quantity.eq(excluded(quantity)),
            average_price.eq(excluded(average_price)),
            updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)
        .await?;
        
    Ok(())
}
```

---

## 🧪 **Testing**

### **Running Tests**
```bash
# Start test database
docker run -d --name postgres-test \
  -e POSTGRES_PASSWORD=test \
  -p 5432:5432 \
  timescale/timescaledb:latest-pg15

# Set test database URL
export DATABASE_URL=postgresql://postgres:test@localhost:5432/test

# Run migrations
diesel setup
diesel migration run

# Run tests
cargo test

# Cleanup
docker stop postgres-test && docker rm postgres-test
```

### **Integration Tests**
```bash
# Run integration tests
cargo test --test integration_tests

# Run with logging
RUST_LOG=debug cargo test
```

---

## 🐳 **Docker**

### **Docker Build**
```bash
# Build database schema container
docker build -t database-schema .

# Run with PostgreSQL
docker run -d --name postgres \
  -e POSTGRES_PASSWORD=secure_password \
  -p 5432:5432 \
  timescale/timescaledb:latest-pg15

docker run --link postgres:database \
  -e DATABASE_URL=postgresql://postgres:secure_password@database:5432/trading_platform \
  database-schema
```

### **Docker Compose**
```yaml
version: '3.8'
services:
  postgres:
    image: timescale/timescaledb:latest-pg15
    environment:
      POSTGRES_PASSWORD: secure_password
      POSTGRES_DB: trading_platform
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"
      
  database-schema:
    build: .
    depends_on:
      - postgres
    environment:
      DATABASE_URL: postgresql://postgres:secure_password@postgres:5432/trading_platform

volumes:
  postgres_data:
```

---

## ☁️ **Deployment**

### **Kubernetes with Helm**
```bash
# Install TimescaleDB
helm repo add timescale https://charts.timescale.com/
helm install timescaledb timescale/timescaledb-single

# Deploy database schema
helm install database-schema ./helm/database-schema \
  --set postgresql.host=timescaledb \
  --set postgresql.password=secure_password
```

### **Production Considerations**
- **High Availability**: Use TimescaleDB clustering or PostgreSQL streaming replication
- **Backup Strategy**: Implement automated daily backups with point-in-time recovery
- **Monitoring**: Deploy PostgreSQL Exporter for Prometheus monitoring
- **Security**: Enable SSL/TLS, rotate passwords, and implement network policies
- **Performance**: Monitor slow queries and optimize indexes for trading workloads

---

## 📊 **Performance Optimization**

### **TimescaleDB Hypertables**
```sql
-- Create hypertable for high-frequency data
SELECT create_hypertable('order_books', 'timestamp', 
                        chunk_time_interval => INTERVAL '1 minute');

-- Enable compression
ALTER TABLE order_books SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'security_id'
);

-- Automated compression policy
SELECT add_compression_policy('order_books', INTERVAL '1 hour');
```

### **Indexing Strategy**
```sql
-- Composite indexes for trading queries
CREATE INDEX idx_order_books_security_time 
ON order_books (security_id, timestamp DESC);

CREATE INDEX idx_trades_execution_time 
ON trades (execution_id, timestamp DESC);

CREATE INDEX idx_positions_account_security 
ON positions (account_id, security_id);
```

---

## 🤝 **Contributing**

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/schema-enhancement`)
3. Run tests (`cargo test`)
4. Create migration if needed (`diesel migration generate`)
5. Commit your changes (`git commit -m 'Add schema enhancement'`)
6. Push to the branch (`git push origin feature/schema-enhancement`)
7. Open a Pull Request

### **Development Setup**
```bash
# Clone repository
git clone https://github.com/Nwagbara-Group-LLC/databaseschema.git
cd databaseschema

# Install dependencies
cargo build

# Setup database
docker run -d --name dev-postgres \
  -e POSTGRES_PASSWORD=dev \
  -p 5432:5432 \
  timescale/timescaledb:latest-pg15

# Run migrations
diesel setup
diesel migration run
```

---

## 📜 **License**

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

---

## 🏢 **About Nwagbara Group LLC**

DatabaseSchema is developed and maintained by Nwagbara Group LLC, providing enterprise-grade database solutions for high-frequency trading platforms. Our systems process billions of database operations daily with microsecond-level performance.

**Contact**: [info@nwagbara-group.com](mailto:info@nwagbara-group.com)

- [`diesel-async`](https://docs.rs/diesel-async)
- [`tokio-postgres`](https://docs.rs/tokio-postgres)
- [`deadpool`](https://docs.rs/deadpool)
- [`native-tls`](https://docs.rs/native-tls)
- [`dotenv`](https://docs.rs/dotenv)
- [`anyhow`](https://docs.rs/anyhow)
- [`tokio-retry`](https://docs.rs/tokio-retry)

---

## 🛠️ Setup

### 1. Clone the Repository

```sh
git clone https://github.com/Nwagbara-Group-LLC/databaseschema.git
cd databaseschema
```

### 2. Add a `.env` File

Create a `.env` file in the project root with your database connection string:

```env
DATABASE_URL=postgres://user:password@hostname:port/database?sslmode=require
```

### 3. Install Rust Dependencies

```sh
cargo build
```

---

## ⚙️ Configuration

The main configuration is via the `DATABASE_URL` environment variable. You can also configure pool size and other parameters in your Rust code or via additional environment variables as needed.

---

## 🚀 Usage

Import the connection pool module and use it to acquire async connections in your application:

```rust
use databaseschema::your_pool_module::get_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let pool = get_pool().await?;
	let mut conn = pool.get().await?;
	// Use conn for queries...
	Ok(())
}
```

---

## 🧩 Example

```rust
// Example: Running a simple query
use databaseschema::your_pool_module::get_pool;
use diesel::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let pool = get_pool().await?;
	let mut conn = pool.get().await?;
	let result = diesel::sql_query("SELECT 1").execute(&mut conn).await?;
	println!("Health check result: {:?}", result);
	Ok(())
}
```

---

## 🗂️ Project Structure

```
databaseschema/
├── src/
│   ├── lib.rs
│   ├── schema.rs
│   ├── models/
│   └── ops/
├── migrations/
├── Cargo.toml
├── diesel.toml
├── README.md
└── ...
```

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Please open an issue or submit a pull request.

---

## 📄 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## 📬 Contact

For questions, feedback, or support, please contact [Nwagbara Group LLC](mailto:info@nwagbara.com) or open an issue on GitHub.
