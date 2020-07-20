FROM rust:1.43.1-slim as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/rustlo-world-api
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/rustlo-world-api /usr/local/bin/rustlo-world-api

EXPOSE 8088

CMD ["rustlo-world-api"]
