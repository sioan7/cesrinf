FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build -p cesrinf-web-docker --release

FROM debian:bookworm-slim AS runner

WORKDIR /app
COPY --from=builder /app/target/release/cesrinf-web-docker /app/
COPY --from=builder /app/cesrinf-web-docker/static /app/
COPY --from=builder /app/cesrinf-web-docker/templates /app/
CMD ["/app/cesrinf-web-docker"]
