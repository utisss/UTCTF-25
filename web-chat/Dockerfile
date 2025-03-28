FROM rust:1.85 as build-env
WORKDIR /app
COPY server /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /app/target/release/chat /
COPY server/assets /assets
CMD ["./chat", "[::]:9000"]

EXPOSE 9000
