# FROM rust:bullseye as builder
# RUN cargo install --locked trunk
# RUN rustup target add wasm32-unknown-unknown

# WORKDIR /app
# RUN mkdir /app/common
# RUN mkdir /app/client
# COPY /common/Cargo.toml ./common/Cargo.toml
# COPY /client/Cargo.toml ./client/Cargo.toml
# WORKDIR /app/common
# RUN apt-get update && apt-get install apt-utils
# RUN DEBIAN_FRONTEND=noninteractive apt-get install -y libwayland-client0
# # RUN apt-get install libwayland-client0
# RUN mkdir src \
#     && echo "// dummy file" > src/lib.rs
# WORKDIR /app/client
# RUN mkdir src \
#     && echo "// dummy file" > src/lib.rs \
#     && cargo build --release
# RUN mkdir style
# RUN mkdir public

# WORKDIR /app
# COPY /common/src ./common/src
# COPY /client/src ./client/src
# COPY /client/index.html ./client
# COPY /client/public ./client/public
# COPY /client/style ./client/style


# WORKDIR /app/client
# RUN TRUNK_PROD=true trunk build --release

FROM nginx:alpine
COPY mime.types /etc/nginx
COPY /client/dist /usr/share/nginx/html
# COPY --from=builder /app/client/dist /usr/share/nginx/html
