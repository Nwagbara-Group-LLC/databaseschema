# 🗃️ TimescaleDB Async Connection Pool (Rust)

This Rust library provides a secure, resilient, and efficient way to manage a pool of asynchronous PostgreSQL (TimescaleDB) connections using `diesel-async`, `tokio-postgres`, and `deadpool`. It is designed for high-performance, production-grade applications that require robust database connectivity, TLS security, and seamless integration with async Rust ecosystems.

---

## 📖 Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Dependencies](#dependencies)
- [Setup](#setup)
- [Configuration](#configuration)
- [Usage](#usage)
- [Example](#example)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

---

## ✨ Features

- 🔁 **Asynchronous PostgreSQL (TimescaleDB) connection pooling**
- 🔐 **Secure TLS connections** using `native-tls`
- ✅ **Retry logic** for acquiring connections (`tokio-retry`)
- ♻️ **Connection recycling** with health check (`SELECT 1`)
- 📦 **Easy configuration** via `.env` and `dotenv`
- 🛠️ **Modular structure** for extensibility (`models`, `schema`, `ops`)
- 📊 **TimescaleDB compatibility** for time-series data
- 🧪 **Testable and production-ready**

---

## 🏗️ Architecture

This library leverages the following core components:

- **`diesel-async`**: Async ORM for Rust, providing type-safe query building and execution.
- **`tokio-postgres`**: Async PostgreSQL driver for low-level database operations.
- **`deadpool`**: Connection pool manager for efficient resource utilization.
- **`native-tls`**: Enables secure TLS/SSL connections to PostgreSQL/TimescaleDB.
- **`tokio-retry`**: Implements retry logic for robust connection acquisition.
- **`dotenv`**: Loads environment variables from a `.env` file for easy configuration.

---

## 📦 Dependencies

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
