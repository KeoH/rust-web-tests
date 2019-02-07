FROM rust:1.32.0-slim-stretch

WORKDIR /usr/src/myweb

COPY . .
RUN rustup default nightly
RUN cargo install --path .

EXPOSE 3000

CMD ["myweb"]