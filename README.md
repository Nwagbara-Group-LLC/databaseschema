# 🗃️ DatabaseSchema - Trading Platform Database Library



**PostgreSQL database library and migrations for the Trading Platform.**



[![Rust](https://img.shields.io/badge/rust-1.82+-orange.svg)](https://www.rust-lang.org)**Rust library providing PostgreSQL schema and migrations for the Trading Platform.**[![Rust](https://img.shields.io/badge/rust-1.82+-orange.svg)](https://www.rust-lang.org)

[![PostgreSQL](https://img.shields.io/badge/postgresql-15+-blue.svg)](https://www.postgresql.org)

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)[![PostgreSQL](https://img.shields.io/badge/postgresql-15+-blue.svg)](https://www.postgresql.org)



---## What is this?[![TimescaleDB](https://img.shields.io/badge/timescaledb-enabled-green.svg)](https://www.timescale.com)



## What is this?[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)



This is a **library**, not a standalone service. It has two purposes:This is a **library**, not a standalone service. It serves two purposes:



1. **Rust Library** - Imported by DataEngine and other services for PostgreSQL connection poolingA **Rust library** providing database schema, connection pooling, and Diesel migrations for the Trading Platform ecosystem.

2. **Migration Runner** - Diesel migrations that run in Kubernetes Jobs

1. **Rust Library**: Imported by DataEngine and other services for PostgreSQL connection pooling

**Important:**

- ❌ NOT a standalone microservice2. **Migration Runner**: Diesel migrations run in a Kubernetes Job---

- ❌ NO HTTP server or API

- ✅ Library used by other services

- ✅ Contains Diesel migrations

## Usage as a Library## 🎯 **What is this?**

---



## Usage as a Library

```toml**DatabaseSchema is a LIBRARY, not a standalone service.**

```toml

# Cargo.toml# Cargo.toml

[dependencies]

databaseschema = { path = "../databaseschema" }[dependencies]It serves two purposes:

```

databaseschema = { path = "../databaseschema" }1. **Rust Library**: Imported by DataEngine and other services for PostgreSQL connection pooling

```rust

use databaseschema::{create_timescale_connection_pool, get_timescale_connection};```2. **Migration Runner**: Contains Diesel migrations that run in a Kubernetes Job

use std::sync::Arc;



#[tokio::main]

async fn main() {```rust### **Important**

    let pool = Arc::new(create_timescale_connection_pool());

    let conn = get_timescale_connection(pool).await.unwrap();use databaseschema::{create_timescale_connection_pool, get_timescale_connection};- ❌ **NOT** a standalone microservice

    // Use connection for database operations

}use std::sync::Arc;- ❌ **NO** HTTP server or API

```

- ✅ **Library** used by other services

---

#[tokio::main]- ✅ **Contains** Diesel migrations for database schema

## Running Migrations

async fn main() {

### Local Development

    let pool = Arc::new(create_timescale_connection_pool());---

```bash

# Install diesel CLI    let conn = get_timescale_connection(pool).await.unwrap();

cargo install diesel_cli --no-default-features --features postgres

    // Use connection...## � **Usage**

# Run migrations

diesel migration run}



# Rollback last migration```### **As a Library (in other Rust projects)**

diesel migration revert

```



### Production (Kubernetes)## Running Migrations```rust



Migrations run automatically via GitHub Actions and Kubernetes Jobs.use databaseschema::{create_timescale_connection_pool, get_timescale_connection};



The workflow builds a Docker image that runs `diesel migration run`:### Local Developmentuse std::sync::Arc;



```yaml

# Deployed via k8s/migrations.yaml

apiVersion: batch/v1```bash#[tokio::main]

kind: Job

metadata:# Install diesel CLIasync fn main() {

  name: database-migrations

spec:cargo install diesel_cli --no-default-features --features postgres    // Create connection pool

  template:

    spec:    let pool = Arc::new(create_timescale_connection_pool());

      containers:

      - name: migrations# Run migrations    

        image: ghcr.io/nwagbara-group-llc/databaseschema:latest

        env:diesel migration run    // Get connection from pool

        - name: DATABASE_URL

          valueFrom:```    let conn = get_timescale_connection(pool).await.unwrap();

            secretKeyRef:

              name: database-secret    

              key: url

```### Kubernetes (Production)    // Use connection...



---}



## ConfigurationMigrations run automatically via a Kubernetes Job using the `databaseschema` Docker image:```



Create a `.env` file (see `.env.example`):



```bash```bash### **Running Migrations (Kubernetes Job)**

DATABASE_URL=postgres://user:password@host:5432/database?sslmode=require

```kubectl apply -f k8s/migrations.yaml



**Never commit the `.env` file!** It contains credentials.```The Docker image is used ONLY for running migrations:



---



## Database SchemaThe Job executes `diesel migration run` against your PostgreSQL instance.```yaml



The library provides:# Kubernetes Job



- **TimescaleDB hypertables** for time-series market data## ConfigurationapiVersion: batch/v1

- **PostgreSQL with PostGIS** for geospatial data

- **Diesel ORM** for type-safe querieskind: Job

- **Deadpool** for async connection pooling

Set `DATABASE_URL` in your environment:metadata:

### Core Tables

  name: database-migrations

- `securities` - Security master data (symbols, exchanges)

- `order_books` - Real-time order book data (TimescaleDB hypertable)```bashspec:

- `trades` - Trade execution history

- `positions` - Portfolio positions# .env file (NEVER commit this!)  template:

- `backtest_results` - Backtesting results

DATABASE_URL=postgres://user:password@host:5432/database?sslmode=require    spec:

---

```      containers:

## CI/CD

      - name: migrations

### Automatic Image Builds

## Dependencies        image: ghcr.io/nwagbara-group-llc/databaseschema:latest

GitHub Actions automatically builds and pushes to GHCR when you:

- Change migrations        env:

- Update source code

- Modify Dockerfile- PostgreSQL 15+ with PostGIS extension        - name: DATABASE_URL



**No manual steps needed!** Just `git push`.- TimescaleDB (for time-series data)          value: "postgres://user:pass@postgresql:5432/dbname"



### Image Location- Diesel ORM for migrations and query builder        # Default CMD is: diesel migration run



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
