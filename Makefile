# Variables
CARGO=cargo
BUILD_DIR=target/lambda
BINARIES=lambda-post-article

# Default target
all: build

# Build all Lambda functions
build:
	@for bin in $(BINARIES); do \
		$(CARGO) lambda build --release --bin $$bin; \
	done

# Clean up build
clean:
	$(CARGO) clean


run-post:
	$(CARGO) lambda invoke --bin lambda-post-article --data-file events/post.json

