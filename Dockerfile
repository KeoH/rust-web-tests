FROM rust:1.32.0-slim-stretch

WORKDIR /usr/src/variables

COPY . .

RUN rustup default nightly && rustup update && cargo update

RUN cargo install --path .

EXPOSE 8000

CMD ["variables"]