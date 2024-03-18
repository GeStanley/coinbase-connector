FROM rust:1.76 as builder

WORKDIR /usr/src/coinbase-connector
COPY . .

RUN cargo install --path .

CMD ["coinbase-connector"]