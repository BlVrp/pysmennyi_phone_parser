# Variables
CARGO = cargo
RUSTUP = rustup

build:
	@echo "Building the project..."
	$(CARGO) build

release:
	@echo "Building the project in release mode..."
	$(CARGO) build --release

run:
	@echo "Running the project..."
	$(CARGO) run

run-release:
	@echo "Running the project in release mode..."
	$(CARGO) run --release

clean:
	@echo "Cleaning the project..."
	$(CARGO) clean

test:
	@echo "Running tests..."
	$(CARGO) test

fmt:
	$(CARGO) fmt -- --check

format:
	@echo "Formatting code..."
	$(CARGO) fmt

clippy:
	@echo "Running Clippy linter..."
	$(CARGO) clippy

update:
	@echo "Updating Rust toolchain..."
	$(RUSTUP) update

setup:
	$(RUSTUP) component add clippy rustfmt

all: build test

help:
	@echo "Available targets:"
	@echo "  build        - Build the project"
	@echo "  release      - Build the project in release mode"
	@echo "  run          - Run the project"
	@echo "  run-release  - Run the project in release mode"
	@echo "  clean        - Clean the project"
	@echo "  test         - Run tests"
	@echo "  fmt          - Check code formatting"
	@echo "  format       - Format code"
	@echo "  clippy       - Run Clippy linter"
	@echo "  update       - Update Rust toolchain"
	@echo "  setup        - Install Clippy and other tools"
	@echo "  all          - Build and test the project"
	@echo "  help         - Show this help menu"
