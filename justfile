VERSION := `toml get Cargo.toml workspace.package.version | jq -r`
export TAG:=`toml get Cargo.toml "workspace.package.version" | jq -r .`

# List available commands
_default:
	just --choose --chooser "fzf +s -x --tac --cycle"

help:
	just --list

test:
	cargo nextest run --no-fail-fast

# Generate usage samples
usage:
	cargo run -q -- --help > doc/cli/usage.adoc
	cargo run -q -- generate --help > doc/cli/generate.adoc
	cargo run -q -- scan --help > doc/cli/scan.adoc
	cargo run -q -- check --help > doc/cli/check.adoc
	cargo run -q -- load --help > doc/cli/load.adoc

# Build the Rust doc
rustdoc:
	./scripts/build-doc.sh

# Generate documentation
doc: rustdoc usage

# Run rustfmt
fmt:
	cargo +nightly fmt --all

# Run clippy
clippy:
	cargo +nightly clippy --features="v14" --all-targets --tests

# Run checks such as clippy, rustfmt, etc...
check: clippy fmt

# Generate the readme as .md
md:
	#!/usr/bin/env bash
	asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md

tag:
	#!/usr/bin/env bash
	echo "Tagging version v$TAG"
	git tag "v$TAG" -f
	git tag | sort -Vr | head

tag-push:
	#!/usr/bin/env bash
	echo "Pushing version v$TAG"
	git push origin "v$TAG"

# Build container using podman by default
container-build:
	#!/usr/bin/env bash
	ENGINE=${ENGINE:-podman}
	$ENGINE build -t prdoc:v$TAG -t paritytech/prdoc -t docker.io/paritytech/prdoc .
	$ENGINE run --rm -it -v $PWD:/repo  prdoc --version
	$ENGINE images | grep prdoc

container-check:
	#!/usr/bin/env bash
	ENGINE=${ENGINE:-podman}
	$ENGINE run --rm -it -v $PWD:/repo  prdoc --help
	$ENGINE run --rm -it -v $PWD:/repo  prdoc --version

# Watch and hot-reload the rustdoc
rustdoc_watch:
	cargo watch -s './scripts/build-doc.sh && browser-sync start --ss target/doc -s target/doc --directory --no-open'
