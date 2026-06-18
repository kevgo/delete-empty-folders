RTA_VERSION = 0.34.0

CUCUMBER_SORT = $(RTA) cucumber-sort
RTA = tools/rta@${RTA_VERSION}
RUMDL = $(RTA) rumdl

build:  # builds the codebase
	cargo build

cuke: build  # run end-to-end tests
	rm -rf tmp
	cargo test --test=cucumber

cukethis: build  # runs only end-to-end tests with a @this tag
	cargo test --test=cucumber -- -t @this

fix: ${RTA} # correct all auto-fixable issues
	cargo +nightly fix --allow-dirty
	cargo clippy --fix --allow-dirty
	cargo +nightly fmt
	$(CUCUMBER_SORT) format
	$(RUMDL) fmt --quiet

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT:' | grep -v help | grep -v '[$$]{RTA}:' | sed 's/:.*#/#/' | column -s "#" -t

lint:  # run all linters
	cargo clippy --all-targets --all-features -- --deny=warnings
	cargo clippy --test=cucumber --all-features -- --deny=warnings
	$(CUCUMBER_SORT) check
	$(RUMDL) check

ps: fix lint cuke  # pitstop

setup:  # install development dependencies on this computer
	rustup component add clippy
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	cargo install cargo-machete --locked

test: lint cuke  # run all tests

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

${RTA}:
	rm tools/rta@* 2>/dev/null || true
	curl -fSL https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh -s -- --version ${RTA_VERSION} --name $(RTA)

.SILENT:
.DEFAULT_GOAL := help
