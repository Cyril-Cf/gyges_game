FROM rust:latest
RUN rustup target add wasm32-unknown-unknown
# RUN cargo install --locked trunk
RUN wget -qO- https://github.com/trunk-rs/trunk/archive/refs/tags/v0.19.0-alpha.1.tar.gz | tar -xzf-
WORKDIR /app
COPY . .
EXPOSE 8080

# CMD ["cargo", "watch", "-x", "trunk serve --address 0.0.0.0"]


CMD ["trunk", "serve", "--address", "0.0.0.0"]