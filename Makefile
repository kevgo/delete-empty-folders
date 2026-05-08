RTA_VERSION = 0.34.0

RTA = tools/rta@${RUN_THAT_APP_VERSION}

cuke:
	rm -rf tmp
	cargo test --test=cucumber

lint:
	cargo clippy --all-targets --all-features -- --deny=warnings

test: lint cuke

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

${RTA}:
	rm .tools/rta@* 2>/dev/null || true
	curl -fSL https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh -s -- --version ${RTA_VERSION} --name $(RTA)

.SILENT:
.DEFAULT_GOAL := help
