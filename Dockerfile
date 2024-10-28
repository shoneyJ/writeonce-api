# Stage 1: Build the Rust application
FROM rust:latest as builder

ARG PORT
ARG AWS_INFRA_BASE_URL

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*


# Install and configure sccache to speed up compilation
RUN cargo install sccache
ENV RUSTC_WRAPPER="sccache"

# Set the working directory inside the container
WORKDIR /usr/src/app

# Cache dependencies to speed up builds
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf target/release/deps/*

# Copy the entire project into the container
COPY . .

# Build the application in release mode
# RUN cargo install --path .

RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libc6 \
    && rm -rf /var/lib/apt/lists/*


COPY --from=builder /usr/src/app/target/release/writeonce-manage-article-api /usr/local/bin/app


ENV PORT=$PORT
ENV AWS_INFRA_BASE_URL=$AWS_INFRA_BASE_URL
ENV API_ACCESS_TOKEN=$API_ACCESS_TOKEN

# Expose the application port
EXPOSE ${PORT}

# Run the application
CMD ["app"]
