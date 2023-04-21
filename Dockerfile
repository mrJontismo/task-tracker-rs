FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8080

VOLUME /app/data

CMD ["./target/release/task-tracker"]