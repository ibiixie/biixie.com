FROM rust:1 AS builder
COPY ./backend /app
WORKDIR /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/backend /
CMD ["./backend"]
