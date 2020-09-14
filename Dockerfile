FROM rust:1.45.1
ADD Cargo.toml /app/Cargo.toml
RUN cd /app && mkdir src && echo "fn main() { }" > src/main.rs && cargo build --release
ADD src app/src
RUN cd /app && touch src/*.rs && cargo build --release
ENTRYPOINT [ "/app/target/release/k8s" ]