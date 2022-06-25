FROM rust AS build

COPY . /app

RUN cd /app && cargo build

FROM rust

RUN apt update && apt install -y lldb

COPY --from=build /app/target/debug/rweb /app/rweb
COPY html /app/html
WORKDIR /app
ENTRYPOINT [ "/app/rweb" ]