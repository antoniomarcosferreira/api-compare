FROM rust:latest
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.toml
COPY src src
RUN cargo build --release
RUN cargo install --path .
EXPOSE 2000
CMD ["api-service"]