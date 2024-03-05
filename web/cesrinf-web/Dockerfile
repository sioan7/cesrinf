FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build -p cesrinf-web --release

FROM debian:bookworm-slim AS runner

WORKDIR /app
COPY --from=builder /app/target/release/cesrinf-web /app/
COPY --from=builder /app/cesrinf-web/static /app/
COPY --from=builder /app/cesrinf-web/templates /app/
CMD ["/app/cesrinf-web"]
