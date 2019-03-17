FROM rustlang/rust:nightly as builder
ARG project_name
RUN apt-get update && \
    apt-get install musl-tools -y && \
    rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/${project_name}
COPY Cargo.toml Cargo.toml
RUN mkdir src/ && \
    echo "fn main(){}" > src/main.rs && \
    RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl && \
    rm -rf src/ && \
    rm -rf /usr/src/${project_name}/target/x86_64-unknown-linux-musl/release/${project_name} && \
    rm -rf /usr/src/${project_name}/target/x86_64-unknown-linux-musl/release/deps/${project_name}* && \
    rm -rf /usr/src/${project_name}/target/x86_64-unknown-linux-musl/release/${project_name}.d
COPY src src
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
ARG project_name
RUN apk add --no-cache libpq
WORKDIR /root/
COPY --from=builder /usr/src/${project_name}/target/x86_64-unknown-linux-musl/release/${project_name} app
ENV RUST_BACKTRACE 1
EXPOSE 8000
CMD ["./app"]