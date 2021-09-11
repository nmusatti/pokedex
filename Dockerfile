FROM rust:1.55-buster as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin pokedex

FROM rust:1.55-buster as runtime
WORKDIR /app
COPY --from=builder /app/target/release/pokedex /usr/local/bin
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/pokedex"]
