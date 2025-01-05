FROM rust:latest

WORKDIR /usr/src/issue_quest
COPY . .

ENV ROCKET_CONFIG=Rocket.toml

RUN cargo build --release

CMD ["./target/release/issue_quest"]
