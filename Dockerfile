FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8080

VOLUME /app/data

# Labels for traefik
LABEL traefik.enable="true"
LABEL traefik.http.routers.task-tracker-rs.entrypoints="web, websecure"
LABEL traefik.http.routers.task-tracker-rs.rule='Host(`rastit.nurminen.io`)'
LABEL traefik.http.routers.task-tracker-rs.tls="true"
LABEL traefik.http.routers.task-tracker-rs.tls.certresolver="production"

CMD ["./target/release/task-tracker"]