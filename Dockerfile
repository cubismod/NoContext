FROM rust:slim-buster

WORKDIR /usr/src/

# 1. Create a new empty shell project
RUN cargo new --bin nocontext
WORKDIR /usr/src/nocontext

# 2. Copy our manifests and toml file
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./secrets.toml ./secrets.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/nocontext*
RUN cargo install --path .

CMD ["nocontext"]