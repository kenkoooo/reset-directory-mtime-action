FROM rust:1.61 AS builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:buster
COPY --from=builder /app/target/release/reset-directory-mtime-action /usr/bin/reset-directory-mtime-action

ENTRYPOINT ["/usr/bin/reset-directory-mtime-action"]
