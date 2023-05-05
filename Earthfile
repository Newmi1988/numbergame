VERSION 0.7
FROM rust:1.68
WORKDIR /rustexample

install-chef:
   RUN cargo install --debug cargo-chef

prepare-cache:
    FROM +install-chef
    COPY --dir src Cargo.lock Cargo.toml .
    RUN cargo chef prepare
    SAVE ARTIFACT recipe.json

# Using cutoff-optimization to ensure cache hit (see examples/cutoff-optimization)
build-cache:
    FROM +install-chef
    COPY +prepare-cache/recipe.json ./
    RUN cargo chef cook --release
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home


build:
    COPY --dir src Cargo.lock Cargo.toml .
    COPY +build-cache/cargo_home $CARGO_HOME
    COPY +build-cache/target target
    RUN cargo build --release --bin numbergame
    SAVE ARTIFACT target/release/numbergame numbergame

test:
    FROM +build
    COPY +build-cache/cargo_home $CARGO_HOME
    COPY +build-cache/target target
    RUN cargo test

image:
    FROM debian:buster-slim
    COPY +build/numbergame /usr/local/bin/numbergame
    COPY ./game.yml .
    ARG USERNAME=nonroot
    ARG USER_UID=1000
    ARG USER_GID=$USER_UID
    # Create the user
    RUN groupadd --gid $USER_GID $USERNAME \
        && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME
    ENTRYPOINT ["/usr/local/bin/numbergame"]
    USER $USERNAME
    SAVE IMAGE earthly/numbergame:latest

