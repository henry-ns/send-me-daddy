FROM rust:latest as build

RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release && rn src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/app*
RUN cargp build --release

FROM debia:buster-slim

COPY --from=build /app/target/release/app .

CMD ["app"]