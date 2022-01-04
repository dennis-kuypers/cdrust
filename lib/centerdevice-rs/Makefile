all: check build test

todos:
	rg --vimgrep -g '!Makefile' -i todo 

check:
	cargo check --all --examples --tests --benches

build:
	cargo check --all --examples

test:
	cargo test --all --no-fail-fast

clean-package:
	cargo clean -p $$(cargo read-manifest | jq -r .name)

release: clean-package release-test release-bump all
	git commit -am "Bump to version $$(cargo read-manifest | jq .version)"
	git tag v$$(cargo read-manifest | jq -r .version)

release-test: check test clippy
	cargo audit
	cargo +nightly fmt -- --check
	cargo publish --dry-run --allow-dirty

release-bump:
	cargo bump

publish:
	git push && git push --tags

clippy:
	cargo clippy --all --all-targets -- -D warnings $$(source ".clippy.args")

fmt:
	cargo +nightly fmt --all

duplicate_libs:
	cargo tree -d

_update-clippy_n_fmt:
	rustup update
	rustup component add clippy
	rustup component add rustfmt --toolchain=nightly

_cargo_install:
	cargo install -f cargo-tree
	cargo install -f cargo-bump
	cargo install -f cargo-audit

.PHONY: tests

