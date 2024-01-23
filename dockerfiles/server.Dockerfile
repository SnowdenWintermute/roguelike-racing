FROM rust:1.70.0 as builder

WORKDIR /app
RUN mkdir /app/common
RUN mkdir /app/server
COPY /common/Cargo.toml ./common/Cargo.toml
COPY /server/Cargo.toml ./server/Cargo.toml
WORKDIR /app/common
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs
WORKDIR /app/server
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build --release

WORKDIR /app
COPY /common/src ./common/src
COPY /server/src ./server/src

WORKDIR /app/server
RUN cargo build --release
RUN echo LISTING FILES
RUN ls

# FROM rust:1.70.0:alpine
# WORKDIR /app
# COPY --from=builder /app/server/target .
# RUN cargo run
