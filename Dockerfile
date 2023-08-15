FROM rust:latest
WORKDIR app
ADD . /app
EXPOSE 80
RUN cargo build --release
ENTRYPOINT ["target/release/pokemon-api"]