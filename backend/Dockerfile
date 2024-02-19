FROM lukemathwalker/cargo-chef:latest-rust-1.76 AS chef
WORKDIR /usr/src/backend

FROM chef AS prepare
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=prepare /usr/src/backend/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust AS run
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=build /usr/src/backend/target/release/backend .
COPY ./memories .
CMD ["./backend"]