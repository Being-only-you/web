# Use Rust nightly with Alpine as the base image
FROM rustlang/rust:nightly-alpine as builder

# Install necessary packages and tools
RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen \
    # Install Supervisord
    && apk add --no-cache supervisor

# Install npm packages
RUN npm install -g sass

# Install cargo-leptos
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/0.2.5/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

# Build the Rust application
RUN cargo leptos build --release -vv

# Add SurrealDB
RUN apk add --no-cache surrealdb

# Create a directory for SurrealDB data
RUN mkdir -p /data

# Create a supervisord configuration file
COPY supervisord.conf /etc/supervisord.conf

# Final stage
FROM rustlang/rust:nightly-alpine

WORKDIR /app

# Copy the built Rust application and other files
COPY --from=builder /work/target/release/leptos-railway /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/
COPY --from=builder /data /data
COPY --from=builder /etc/supervisord.conf /etc/supervisord.conf

# Expose ports
EXPOSE 8000 8001 $PORT

# Set environment variables
ENV LEPTOS_SITE_ROOT=./site
ENV SURREALDB_URL=http://localhost:8000

# Start supervisord
CMD ["supervisord", "-c", "/etc/supervisord.conf"]
