# syntax=docker/dockerfile:1
# Fat E2E image: every package manager that coexists cleanly on Debian.
# Skipped here:
#   - snapcraft (needs systemd)
#   - macports (macOS only)
#   - brew, cargo, mise, asdf, pyenv, rbenv (slow installs / login-shell
#     bootstrap; covered in a follow-up iteration)
FROM rust:slim AS builder
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:stable-slim

ENV DEBIAN_FRONTEND=noninteractive
ENV LANG=C.UTF-8

# 1. Base tools and several managers in one apt layer.
RUN apt-get update && apt-get install -y --no-install-recommends \
        bash ca-certificates curl git build-essential file \
        jq \
        python3 python3-pip pipx \
        ruby-full \
        golang \
        unzip xz-utils \
    && rm -rf /var/lib/apt/lists/*

# 2. Node.js 24 via NodeSource → gives `node` and `npm`. corepack is
#    deprecated upstream, so install pnpm via its standalone installer
#    instead. (yarn is intentionally skipped for now — Berry has no
#    `global` and classic-yarn-via-npm collides with the npm test.)
RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

ENV PNPM_HOME="/root/.local/share/pnpm"
ENV PATH="${PNPM_HOME}:${PATH}"
RUN curl -fsSL https://get.pnpm.io/install.sh | env SHELL=/bin/bash sh -

# 3. uv (standalone installer) → /root/.local/bin/uv
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/root/.local/bin:${PATH}"

# 4. bun (official installer) → /root/.bun/bin/bun
RUN curl -fsSL https://bun.sh/install | bash
ENV BUN_INSTALL="/root/.bun"
ENV PATH="/root/.bun/bin:${PATH}"

# 5. nvm under /root/.nvm. Exported via NVM_DIR so `how`'s nvm detection
#    (which reads $NVM_DIR or $HOME/.nvm) finds it without a login shell.
ENV NVM_DIR="/root/.nvm"
RUN mkdir -p "$NVM_DIR" \
    && curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash

COPY --from=builder /src/target/release/how /usr/local/bin/how
COPY tests/e2e/cases/debian.sh /opt/cases/debian.sh
RUN chmod +x /opt/cases/debian.sh

CMD ["/opt/cases/debian.sh"]
