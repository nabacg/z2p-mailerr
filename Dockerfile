# heavily inspired by https://github.com/LukeMathWalker/cargo-chef#how-to-use

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
# required for the linking configuration we use
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application, up to this stage everything should be cached if we only change application code
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin z2p-mailerr



FROM debian:bullseye-slim as runtime
RUN apt-get update -y
# Install OpenSSL since it's deynamically linked by some of our deps
RUN apt-get install -y --no-install-recommends openssl ca-certificates 
# Install ca-certificates required to verify TLS certs when establishing HTTPS connections
RUN apt-get install -y --no-install-recommends ca-certificates 
# clean up
RUN apt-get autoremove -y 
RUN apt-get clean -y
RUN rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/z2p-mailerr z2p-mailerr
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./z2p-mailerr"]
