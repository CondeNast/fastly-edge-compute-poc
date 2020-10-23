# Fastly Compute@Edge Rust dev environment

FROM rust:1

ARG RUST_VERSION=1.46.0
ARG FASTLY_CLI_VERSION=0.17.0

ENV RUSTUP_HOME=/usr/local/rustup \
  CARGO_HOME=/usr/local/cargo \
  PATH=/usr/local/cargo/bin:$PATH

WORKDIR /workspace

# # Install rust
# RUN  echo "Installing Rust..." && \
#   curl -fLSs --tlsv1.2 https://sh.rustup.rs | bash -s -- -y --no-modify-path --profile minimal && \
#   rustup update && \
#   rustup component add rls rust-analysis rust-src rustfmt clippy && \
#   rustup toolchain add ${RUST_VERSION} && \
#   rustup target add wasm32-wasi --toolchain ${RUST_VERSION}

# Install rust toolchain
RUN  echo "Installing Rust toolchain..." && \
  rustup update && \
  rustup component add rls rust-analysis rust-src rustfmt clippy && \
  rustup toolchain add ${RUST_VERSION} && \
  rustup target add wasm32-wasi --toolchain ${RUST_VERSION}

# Install Fastly CLI https://github.com/fastly/cli#linux
RUN echo "Installing Fastly CLI..." && \
  curl -fLSs -o ./fastly_cli.deb https://github.com/fastly/cli/releases/download/v${FASTLY_CLI_VERSION}/fastly_${FASTLY_CLI_VERSION}_linux_amd64.deb && \
  dpkg -i ./fastly_cli.deb && \
  rm fastly_cli.deb && \
  fastly version

# Install CircleCI CLI
RUN curl -fLSs https://raw.githubusercontent.com/CircleCI-Public/circleci-cli/master/install.sh | bash
