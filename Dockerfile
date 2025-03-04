FROM rust:1.81-alpine

# ensures essential build tools installed
RUN apk add --no-cache build-base

WORKDIR /usr/src/sql-optimisation-caching-experimenting

COPY Cargo.toml Cargo.lock ./
RUN mkdir src
COPY src/ src/

RUN cargo build --release

CMD ["/usr/src/sql-optimisation-caching-experimenting/target/release/sql-optimisation-caching-experimenting"]
