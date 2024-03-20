FROM rust:latest
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
WORKDIR /app
COPY . .
EXPOSE 8080
CMD ["trunk", "serve", "--address", "0.0.0.0"]