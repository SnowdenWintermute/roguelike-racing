FROM rust:1.76.0 as builder

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
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release

WORKDIR /app
COPY /common/src ./common/src
COPY /server/src ./server/src

WORKDIR /app/common/src
RUN rm -rf target

WORKDIR /app/server
RUN rm -rf target
RUN cargo build --release
RUN echo LISTING FILES
RUN ls

FROM rust:1.76.0 
WORKDIR /app
COPY --from=builder /app/server/target .
RUN ls -a
WORKDIR /
EXPOSE 8082
ENV RUST_BACKTRACE=1
CMD ["./app/release/server"]
