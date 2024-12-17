FROM rust:latest

WORKDIR /usr/src/databaseschema

COPY . .

CMD ["cargo", "build", "--release"]