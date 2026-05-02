# syntax=docker/dockerfile:1
# E2E image for the dnf package manager.
FROM rust:slim AS builder
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM fedora:latest
RUN dnf install -y bash jq && dnf clean all
COPY --from=builder /src/target/release/how /usr/local/bin/how
COPY tests/e2e/cases/fedora.sh /opt/cases/fedora.sh
RUN chmod +x /opt/cases/fedora.sh
CMD ["/opt/cases/fedora.sh"]
