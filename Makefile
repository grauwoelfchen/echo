COVERAGE_REGEXP := 'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | sed -E 's/[a-z\:"]*//g'

# vet -- {{{
vet\:check: # Check rust syntax [synonym: check]
	@cargo check --all -v
.PHONY: vet\:check

check: vet\:check
.PHONY: check

vet\:format: # Check format without changes [synonym: vet:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: vet\:format

vet\:fmt: vet\:format
.PHONY: vet\:fmt

format: vet\:format
.PHONY: format

fmt: vet\:format
.PHONY: fmt

vet\:lint: # Check coding style using clippy [synonym: lint]
	@cargo clippy --all-targets
.PHONY: vet\:lint

lint: vet\:lint
.PHONY: lint

vet\:all: vet\:check vet\:format vet\:lint # Check code using all vet:xxx targets
.PHONY: vet\:all

vet: vet\:all # Alias of vet:all
.PHONY: vet
# }}}

# test -- {{{
test\:all: # Run unit tests and integration tests
	@cargo test --tests
.PHONY: test

test: test\:all # Alias of test:all
.PHONY: test

coverage: # Generate coverage report of unit tests only for lib using kcov [synonym: cov]
	@cargo test --tests --no-run
	@kcov --verify --include-path=src target/coverage target/debug/deps/echo-*
	@grep 'index.html' target/coverage/index.js* | \
		grep --only-matching --extended-regexp $(COVERAGE_REGEXP)
.PHONY: coverage

cov: coverage
.PHONY: cov
# }}}

# build -- {{{
build\:debug: # Run debug build [synonym: debug]
	cargo build
.PHONY: build\:debug

debug: build\:debug
.PHONY: debug

build: build\:debug # Alias of build:debug
.PHONY: build

build\:release: # Create release build [synonym: release]
	cargo build --release
.PHONY: build\:release

release: build\:release
.PHONY: release
# }}}

# utility -- {{{
serve: # Run development server
	@HOST=127.0.0.1 PORT=8000 cargo run --quiet --bin echo
.PHONY: serve

clean: # Remove cache and built artifacts
	@cargo clean
.PHONY: clean

runner-%: # Run a CI job on local (on Docker)
	@set -uo pipefail; \
	job=$(subst runner-,,$@); \
	opt=""; \
	while read line; do \
		opt+=" --env $$(echo $$line sed -E 's/^export //')"; \
	done < .env.ci; \
	gitlab-runner exec docker \
		--executor docker \
		--cache-dir /cache \
		--docker-volumes $$(pwd)/.cache/gitlab-runner:/cache \
		--docker-volumes /var/run/docker.sock:/var/run/docker.sock \
		$${opt} $${job}
.PHONY: runner

help: # Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[0-9a-z\:\\\%]+: ' \
		$(firstword $(MAKEFILE_LIST)) | \
		grep --extended-regexp ' # ' | \
		sed --expression='s/\([a-z0-9\-\:\ ]*\): \([a-z0-9\-\:\ ]*\) #/\1: #/g' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;222m%-14s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help
# }}}

.DEFAULT_GOAL = vet\:all
default: vet\:all
