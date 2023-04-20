FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release
RUN echo "0" > Jon.txt && echo "0" > Robin.txt

EXPOSE 8080

CMD ["./target/release/task-tracker"]