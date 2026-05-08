RTA_VERSION = 0.34.0

RTA = tools/rta@${RUN_THAT_APP_VERSION}

test: lint cuke  # run all tests

cuke:  # run end-to-end tests
	rm -rf tmp
	cargo test --test=cucumber

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT:' | grep -v help | grep -v '[$$]{RTA}:' | sed 's/:.*#/#/' | column -s "#" -t

lint:  # run all linters
	cargo clippy --all-targets --all-features -- --deny=warnings

setup:  # install development dependencies on this computer
	rustup component add clippy
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	cargo install cargo-machete --locked

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

${RTA}:
	rm tools/rta@* 2>/dev/null || true
	curl -fSL https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh -s -- --version ${RTA_VERSION} --name $(RTA)

.SILENT:
.DEFAULT_GOAL := help
