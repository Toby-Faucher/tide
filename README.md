# 🌊 Tide

> AI-powered diabetes management — stay in range, stay in control.

Tide is a web application that helps people with diabetes track, understand, and manage their blood glucose levels using AI-driven insights. Built for speed and reliability with a Rust backend.

---

## ✨ Features

> **TODO:** Expand this section as features are built out.

- [ ] Blood glucose logging and history
- [ ] AI-powered pattern recognition and insights
- [ ] Meal and insulin tracking
- [ ] Personalised recommendations
- [ ] Real-time alerts and notifications
- [ ] Dashboard with trends and analytics

---

## 🛠 Tech Stack

| Layer | Technology |
|-------|-----------|
| Web Framework | [Axum](https://github.com/tokio-rs/axum) |
| Async Runtime | [Tokio](https://tokio.rs) |
| Serialization | [Serde](https://serde.rs) |
| Allocator | [jemalloc](https://github.com/tikv/jemallocator) |
| HTTP Compression | [tower-http](https://github.com/tower-rs/tower-http) |

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs) (latest stable)

### Running locally

```bash
git clone https://github.com/Toby-Faucher/tide.git
cd tide
cargo run
```

The server starts on `http://localhost:3000`.

### Health check

```bash
curl http://localhost:3000/health
# {"status":"ok"}
```

### Running benchmarks

```bash
cargo bench
```

---

## 📁 Project Structure

```
tide/
├── src/
│   └── main.rs        # Application entrypoint & routing
├── benches/
│   └── health.rs      # Criterion benchmarks
├── Cargo.toml
└── .gitignore
```

---

## 🔧 Configuration

> **TODO:** Document environment variables (e.g. `DATABASE_URL`, `AI_API_KEY`, `PORT`, `LOG_LEVEL`) once they are defined.

---

## 🚢 Deployment

> **TODO:** Add deployment instructions (Docker, cloud provider, etc.) once infrastructure is decided.

---

## 🤝 Contributing

> **TODO:** Add a `CONTRIBUTING.md` file covering:
> - Branch naming conventions
> - PR process and review guidelines
> - Code style and formatting (`rustfmt`, `clippy`)
> - How to run the test suite
> - Issue templates

In the meantime, feel free to open an issue or start a discussion.

---

## 📄 Licence

This project is licensed under the [MIT Licence](./LICENSE) — © 2026 Tobias Faucher.

---

## ⚠️ Medical Disclaimer

> **TODO:** Add a medical disclaimer clarifying that Tide is not a substitute for professional medical advice, diagnosis, or treatment. This is important for any health-related application.

---

<p align="center">Built with 🦀 Rust &nbsp;·&nbsp; Made for people living with diabetes</p>
