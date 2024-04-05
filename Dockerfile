FROM rust:bookworm as builder
WORKDIR /usr/src/rustdesk-server
COPY . .

RUN cargo install --path .

FROM debian:bookworm
COPY --from=builder /usr/local/cargo/bin/hbbs /usr/local/bin/hbbs
COPY --from=builder /usr/local/cargo/bin/hbbr /usr/local/bin/hbbr
COPY --from=builder /usr/local/cargo/bin/rustdesk-utils /usr/local/bin/rustdesk-utils
WORKDIR /data