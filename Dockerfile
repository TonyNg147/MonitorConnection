FROM rust:1.87.0 AS build
RUN apt update
WORKDIR /artifact
ADD ./ ./
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM rust:1.87.0-alpine
COPY --from=build  /artifact/target/x86_64-unknown-linux-musl/release/htmx /usr/bin
RUN ls /usr/bin | grep htmx
CMD [ "htmx" ]

