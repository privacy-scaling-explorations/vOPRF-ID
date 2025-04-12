# Build stage
FROM rust:1.85-slim

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    nano \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /vOPRF-ID

# Copy project files
COPY . .

# Build the project using workspace
RUN cargo build --release

# Expose the port
EXPOSE 8080

# Default to bash for interactive use
CMD ["/bin/bash"]
