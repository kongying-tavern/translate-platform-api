# Preload dependencies,
# used to speed up repeated builds and reduce traffic consumption of libraries
FROM rust:latest as stage-deps

RUN apt update && apt install -y clang
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add wasm32-wasi
RUN cargo install cargo-make
RUN cargo install wasm-bindgen-cli@0.2.92

COPY ./Cargo.toml /home/Cargo.toml
RUN cargo new --name boot /home/packages/boot
COPY ./packages/boot/Cargo.toml /home/packages/boot/Cargo.toml
RUN cargo new --lib --name database /home/packages/database
COPY ./packages/database/Cargo.toml /home/packages/database/Cargo.toml

WORKDIR /home
RUN cargo fetch

COPY ./packages/boot /home/packages/boot
COPY ./packages/database /home/packages/database

# Stage 1 for server build, used to compile server program
FROM stage-deps as stage-server-build1

WORKDIR /home
RUN cargo build --offline --package _boot --release

# Stage 2 for server build, used to integrate the build result of client and generate the final image
FROM rust:latest as stage-server-build2

COPY --from=stage-server-build1 /home/target/release/_boot /home/a
ENV ROOT_DIR=/home/res
WORKDIR /home
ENTRYPOINT [ "./a" ]
EXPOSE 80
