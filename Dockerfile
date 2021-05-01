# FRONTEND
FROM node:14-alpine AS frontend

RUN curl -f https://get.pnpm.io/v6.js | node - add --global pnpm

WORKDIR /usr/src/app

COPY fronted ./frontend

WORKDIR /usr/src/app


RUN pnpm install

RUN pnpm build


# BACKEND

FROM rust:1.50-alpine AS builder

WORKDIR /usr/src/app

COPY . .

RUN rustup rustup toolchain install nightly && \
    rustup default nightly

RUN cargo run

EXPOSE 8000

FROM debian:buster-slim


COPY --from=builder /usr/src/target/release/app /app

CMD [ "/app" ]
