.PHONY: all build run test clean

# Default action: Build the project
all: build

# Build the project
build:
	@echo "Building..."
	cargo build

release:
	@echo "Building for release..."
	cargo build --release

# Run the project
run:
	@echo "Running..."
	cargo run

check:
	@echo "Checking..."
	cargo check

# Run tests
test: check
	@echo "Running tests..."
	cargo test

# Clean the project
clean:
	@echo "Cleaning..."
	cargo clean
