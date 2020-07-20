FROM rust:1.45-slim as build
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN USER=root cargo new --bin rustlo-world-api
WORKDIR /rustlo-world-api

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/rustlo_world_api*
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /rustlo-world-api/target/release/rustlo-world-api .

EXPOSE 8088

CMD ["./rustlo-world-api"]
