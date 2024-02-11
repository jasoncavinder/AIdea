# Use a Rust image as base for building the project
FROM rust:latest as builder
WORKDIR /usr/src/aidea

# Copy the source code and Cargo files into the image
COPY . .

# Compile the project, release build for optimized performance
RUN cargo build --release

# Use Debian slim image for smaller final image size
FROM rust:slim-bookworm
COPY --from=builder /usr/src/aidea/target/release/aidea /usr/local/bin/aidea

# Set the command to run the application
CMD ["aidea"]
