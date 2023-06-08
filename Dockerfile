FROM rust:1.70-buster


WORKDIR /app

# required for the linking configuration we use
RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./target/release/z2p-mailerr"]
