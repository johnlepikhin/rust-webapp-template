all: build-fast

###############################################################################
# Generators
###############################################################################

generate-webapp:
	cargo generate -p ./.cargo-generate/templates/webapp

generate-plugin:
	cargo generate -p ./.cargo-generate/templates/plugin

generate-db-postgres:
	cargo generate -p ./.cargo-generate/templates/db_postgres

###############################################################################
# Builders
###############################################################################

build-fast:
	cargo build --profile fastdev

build-debug:
	cargo build

build-release:
	cargo build --release

build-deb:
	cargo deb -p "{{project-name}}"

###############################################################################
# Misc
###############################################################################

run:
	cargo run -p "{{project-name}}" -- run

clean:
	cargo clean

release-tag:
	git tag v`convco version --bump`

release-changelog:
	convco changelog

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

	echo "Installing cargo-deb"
	cargo install cargo-deb

	echo "Installing convco"
	cargo install convco

	echo "Installing ripsecrets"
	cargo install --git https://github.com/sirwart/ripsecrets --branch main

###############################################################################
# Validators
###############################################################################

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

###############################################################################
# Automatic fixups
###############################################################################

fix-clippy:
	cargo clippy --fix

fix-format:
	cargo fmt

fix-audit:
	cargo audit fix

fix-all: fix-clippy fix-format fix-audit
