# 🗣️ Voice AI Agent (Dockerized)

A Rust-based Gen-Z-styled AI chatbot server built with `warp`, `diesel`, and `SQLite`, containerized using Docker. It uses Together AI's LLM API to generate witty and casual responses, and logs all interactions to the database.

---

## 🚀 Features

- 🧠 LLM-powered chatbot (Together AI)
- 🎙️ Gen-Z style responses
- 🗂️ Logs all conversations to SQLite
- 🐳 Fully Dockerized
- 🔥 Built with `warp`, `diesel`, `r2d2`, and `tokio`

---

## 📦 Technologies Used

- **Rust** (async + typed safety)
- **Warp** (Fast web framework)
- **Diesel** (Rust ORM with SQLite)
- **r2d2** (Connection pooling)
- **Docker** (Build & deploy)
- **Together AI API** (LLM inference)


---

## ⚙️ Getting Started

### 1. Clone the repo

```bash
git clone https://github.com/Kayleexx/voice_ai_agent.git
cd voice_ai_agent
```

### 2. Set up your environment

```env
# .env
DATABASE_URL=db.sqlite
TOGETHER_AI_KEY=your_api_key
RUST_LOG=debug
```

### 3. Run migrations

```bash
diesel setup
diesel migration run
```

### 4. Run with Docker

```bash
docker build -t voice-ai-agent .
docker run -p 3030:3030 -v $(pwd)/db.sqlite:/app/db.sqlite --name voice-agent voice-ai-agent
```
---

## 🧪 Test the Webhook

```bash
curl -X POST http://localhost:3030/webhook \
  -H "Content-Type: application/json" \
  -d '{"user_input": "Hello AI"}'
```

---

## ✨ Credits

Built with 💙 by Mitali
