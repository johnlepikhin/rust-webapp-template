all: build-fast

build-fast:
	cargo build --profile fastdev

build-debug:
	cargo build

build-release:
	cargo build --relese

clean:
	cargo clean

release-tag:
	git tag v`convco version --bump`

check-clippy:
	cargo clippy

check-unused-deps:
	cargo udeps

check-outdated-deps:
	cargo outdated

check-audit-deps:
	cargo audit

check-cyclic-deps:
	cargo get workspace.members | while read member; do \
	    echo "Checking cyclic deps in module $$member"; \
		member=$$(echo "$$member" | tr -d '\r'); \
		cargo modules generate graph --layout none --acyclic --package "$$member"; \
	done

check-secrets:
	ripsecrets

check-all: check-clippy check-outdated-deps check-audit-deps check-cyclic-deps ripsecrets

install-tools:
	echo "Installing cargo-get"
	cargo install cargo-get

	echo "Installing cargo-udeps"
	cargo install  cargo-udeps

	echo "Installing cargo-outdated"
	cargo install cargo-outdated

	echo "Installing cargo-modules"
	cargo install  cargo-modules

	echo "installing cargo audit"
	cargo install cargo-audit --features=fix

	echo "Installing ripsecrets"
	cargo install --git https://github.com/sirwart/ripsecrets --branch main

fix-clippy:
	cargo clippy --fix

fix-format:
	cargo fmt

fix-audit:
	cargo audit fix

fix-all: fix-clippy fix-format fix-audit
