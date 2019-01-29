FROM rust:1.32.0-slim-stretch

WORKDIR /usr/src/myweb

COPY . .

RUN cargo install --path .

EXPOSE 3000

CMD ["myweb"]