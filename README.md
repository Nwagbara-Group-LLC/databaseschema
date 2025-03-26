# 🗃️ TimescaleDB Async Connection Pool (Rust)

This Rust library provides a secure, resilient, and efficient way to manage a pool of asynchronous PostgreSQL (TimescaleDB) connections using `diesel-async`, `tokio-postgres`, and `deadpool`. It includes TLS support and retry logic for robust production usage.

---

## ✨ Features

- 🔁 Asynchronous PostgreSQL (TimescaleDB) connection pooling
- 🔐 Secure TLS connections using `native-tls`
- ✅ Retry logic for acquiring connections (`tokio-retry`)
- ♻️ Connection recycling with health check (`SELECT 1`)
- 📦 Easy `.env` integration via `dotenv`
- 🛠️ Modular structure for extensibility (`models`, `schema`, `ops`)

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

### 1. Add `.env` file

```env
DATABASE_URL=postgres://user:password@hostname:port/database?sslmode=require
