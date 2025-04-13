# Build stage
FROM rust:1.76 as builder

# Install required dependencies for diesel
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Create working directory
WORKDIR /usr/src/app

# Copy all source code and configuration
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/suarakan-be /usr/local/bin/suarakan-be

# Copy migrations folder for Diesel
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations
COPY --from=builder /usr/src/app/diesel.toml /usr/local/bin/

# Set working directory
WORKDIR /usr/local/bin

# Expose the port your application runs on
EXPOSE 8080

# Run the binary
CMD ["suarakan-be"]