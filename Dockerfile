# BUILDER
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# User
ENV USER=ghost
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /rusty-ghost

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

# RUNNER
FROM scratch

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /rusty-ghost/target/x86_64-unknown-linux-musl/release/rusty-ghost ./

USER ghost:ghost

EXPOSE 3773

ENTRYPOINT ["/rusty-ghost"]
CMD ["--zero-dl"]
