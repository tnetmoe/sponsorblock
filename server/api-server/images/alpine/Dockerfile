# Alpine 
# https://hub.docker.com/_/alpine
# BUILD CONTEXT HAS TO BE THE PROJECT/REPOSITORY ROOT! ("docker build -f sb/api-server/images/alpine/Dockerfile .")

# Seperate deps image with cargo chef for better caching 
FROM rust:1.79-alpine AS deps
LABEL org.opencontainers.image.source="https://github.com/tnetmoe/sponsorblock"
LABEL org.opencontainers.image.description="SponsorBlock api server"
LABEL org.opencontainers.image.licenses="AGPL-3.0-only"

WORKDIR /deps

# Install system dependencies
RUN apk add --no-cache musl-dev

# Copy the Cargo.toml and Cargo.lock files
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./server/api-server/Cargo.toml ./server/api-server/Cargo.toml

WORKDIR /deps/server/api-server
RUN cargo install cargo-chef --version 0.1.67

# Build the project dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM deps AS builder
WORKDIR /build

# Copy source code
COPY . .
WORKDIR /build/server/api-server
COPY --from=deps /deps/recipe.json /build/server/api-server/recipe.json

# Build the project
RUN cargo chef cook --release --recipe-path ./recipe.json

# Final image
# Use vectors for commands to avoid using shell
# -> https://github.com/tnetmoe/distroless/blob/9fd924863f76e8340af38441573668d80155405f/README.md#entrypoints
FROM alpine:3.19

# Set the working directory
WORKDIR /sb

# Add non-root user
RUN ["groupadd", "-r", "sb", "&&", "useradd", "-r", "-g", "sb", "sb"]

# Copy the built artifacts from the builder stage
COPY --from=builder /build/target/release/sb-api-server .

# Switch to non-root user
USER sb

# Set the entrypoint
CMD ["./sb-api-server"]