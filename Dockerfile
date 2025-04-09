# 1. Use official Rust image for building
FROM rust:latest as builder

# 2. Create app directory
WORKDIR /app

# 3. Copy source files
COPY . .

# 4. Install dependencies for Diesel + SQLite
RUN apt-get update && apt-get install -y pkg-config libssl-dev sqlite3 libsqlite3-dev

# 5. Build in release mode
RUN cargo build --release

# 6. Create minimal runtime image
FROM debian:bookworm-slim

WORKDIR /app

# 7. Copy the binary and necessary files from the builder
COPY --from=builder /app/target/release/voice_ai_agent /app/
COPY --from=builder /app/.env /app/
COPY --from=builder /app/migrations /app/migrations

# 8. Install runtime dependencies (correct OpenSSL version)
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates sqlite3 && \
    rm -rf /var/lib/apt/lists/*

# 9. Set environment variables
ENV RUST_LOG=info

# 10. Run your binary
CMD ["./voice_ai_agent"]
