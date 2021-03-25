FROM rust:1 as builder
WORKDIR /usr/src/circuit
COPY . .
RUN cargo install --path .
RUN strip /usr/local/cargo/bin/backend

FROM debian:stable-slim
EXPOSE 8000
#RUN apt-get update && apt-get install -y any-extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
CMD ["backend"]
