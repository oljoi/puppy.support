FROM docker.io/rustlang/rust:nightly-alpine as builder

RUN apk update && apk add --no-cache bash curl npm libc-dev binaryen

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

RUN rustup target add wasm32-unknown-unknown

RUN mkdir -p /app
WORKDIR /app
COPY . .

RUN cargo leptos build --release -vv

FROM docker.io/rustlang/rust:nightly-alpine as runner

RUN apk update && apk add --no-cache openssl ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/puppy-support /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 3000

CMD ["/app/puppy-support"]
