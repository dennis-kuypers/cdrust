# TODO: currently very fragile and only for x64
# TODO: currently manual bumping of version numbers required

# cli projects to build and pack
CLI_BINS = cssh cvpn

.PHONY: usage
usage:
	@echo "USAGE:"
	@echo "  make gh-release  - compiles files for homebrew/gh-release"
	@echo "  make clean       - cleans /target/..."

clean:
	cargo clean

# keep binaries
.PRECIOUS: target/release/%
target/release/%:
	@echo "compiling $*"
	@cargo b --release -p $*

.PHONY: gh-release
gh-release: $(addprefix homebrew-tar-,$(CLI_BINS))
	ls -l target/homebrew/release/

target/homebrew/%: target/release/%
	@mkdir -p target/homebrew/$*/bin
	@cp target/release/$* target/homebrew/$*/bin/

.PHONY: homebrew-tar-%
homebrew-tar-%: VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name | contains("$*")).version')
homebrew-tar-%: target/release/%
	@echo "homebrew: packing $*-${VERSION}"
	@mkdir -p target/homebrew/release
	@tar -caf target/homebrew/release/$*-${VERSION}-x86_64-apple-darwin.tar.gz -C target/homebrew/$* bin/$*
