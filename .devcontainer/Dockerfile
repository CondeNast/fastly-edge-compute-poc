# Fastly Compute@Edge Rust dev environment

FROM buildpack-deps

ARG RUST_VERSION=1.46.0
ARG FASTLY_CLI_VERSION=0.17.0

ENV RUSTUP_HOME=/usr/local/rustup \
  CARGO_HOME=/usr/local/cargo \
  PATH=/usr/local/cargo/bin:$PATH

# Install rust
RUN  echo "Installing Rust..." && \
  curl --tlsv1.2 https://sh.rustup.rs -sSf | bash -s -- -y --no-modify-path --profile minimal && \
  rustup update && \
  rustup component add rls rust-analysis rust-src rustfmt clippy && \
  rustup toolchain add ${RUST_VERSION} && \
  rustup target add wasm32-wasi --toolchain ${RUST_VERSION}

# Install Fastly CLI https://github.com/fastly/cli#linux
RUN curl -L -o fastly_cli.deb https://github.com/fastly/cli/releases/download/v${FASTLY_CLI_VERSION}/fastly_${FASTLY_CLI_VERSION}_linux_amd64.deb && \
  dpkg -i ./fastly_cli.deb && \
  rm fastly_cli.deb && \
  fastly version

WORKDIR /workspace
