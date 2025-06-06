# Use the official Rust image as the base image for the build stage
FROM rust:1.85 AS builder

# Set the working directory
WORKDIR /usr/src/backend

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new empty shell project to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release
RUN rm -f target/release/deps/backend*

# Now copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller image for the final runtime
FROM debian:bookworm-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/backend
COPY Rocket.toml .

# Expose the port your application runs on
EXPOSE 8000

# Start the application
CMD ["backend"]
