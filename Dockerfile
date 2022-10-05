FROM rust

WORKDIR /code

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src

RUN cargo build

CMD ["cargo", "run"]