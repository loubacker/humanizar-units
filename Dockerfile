FROM rust:1.98.0-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --locked --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system appgroup \
    && useradd --system --gid appgroup --no-create-home appuser

COPY --from=builder --chown=appuser:appgroup /app/target/release/humanizar-units /app/humanizar-units

ENV SERVER_HOST=0.0.0.0 \
    SERVER_PORT=9095

EXPOSE 9095

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD ["curl", "--fail", "--silent", "--show-error", "http://127.0.0.1:9095/health"]

USER appuser

ENTRYPOINT ["/app/humanizar-units"]
