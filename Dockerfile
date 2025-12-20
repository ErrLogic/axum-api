# =========================
# Builder stage
# =========================
FROM rust:slim-bookworm AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

# =========================
# Runtime stage
# =========================
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/axum-api /app/app

RUN useradd -m appuser
USER appuser

EXPOSE 3000

CMD ["./app"]
