FROM rust:1.50-alpine as builder

WORKDIR /usr/src/app

COPY . .

RUN rustup rustup toolchain install nightly && \
    rustup default nightly

RUN cargo run

EXPOSE 8000

FROM debian:buster-slim

ENV ROCKET_ENV=production

COPY --from=builder /usr/src/target/release/app /app

CMD [ "/app" ]
