FROM rust AS build

WORKDIR /app

# Copy Cargo files
COPY ./Cargo.toml .
COPY ./Cargo.lock .

# Create fake main.rs file in src and build
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
# Fake build
RUN cargo build && cargo clean
RUN rm -rf ./src

# now copy source, the layers above have already cached the crates.io index (unless you update Cargo.toml)
COPY . /app

RUN cd /app && cargo build

FROM rust

RUN apt update && apt install -y lldb

COPY --from=build /app/target/debug/rweb /app/rweb
COPY html /app/html
WORKDIR /app
ENTRYPOINT [ "/app/rweb" ]