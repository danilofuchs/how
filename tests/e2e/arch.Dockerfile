# syntax=docker/dockerfile:1
# E2E image for the pacman package manager.
FROM --platform=linux/amd64 rust:slim AS builder
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# archlinux:base is amd64-only; pin platform so this still builds under
# qemu on Apple Silicon. CI runners are amd64, so this is a no-op there.
FROM --platform=linux/amd64 archlinux:base
# pacman's seccomp sandbox fails under qemu emulation on Apple Silicon.
# Harmless on native amd64 (CI), required for local M-series dev.
RUN printf '\nDisableSandbox\n' >> /etc/pacman.conf \
    && pacman -Sy --noconfirm --needed bash jq \
    && pacman -Scc --noconfirm
COPY --from=builder /src/target/release/how /usr/local/bin/how
COPY tests/e2e/cases/arch.sh /opt/cases/arch.sh
RUN chmod +x /opt/cases/arch.sh
CMD ["/opt/cases/arch.sh"]
