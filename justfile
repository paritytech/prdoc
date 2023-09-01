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
	cargo run -q -- schema --help > doc/cli/schema.adoc
	cargo run -q -- generate --help > doc/cli/generate.adoc
	cargo run -q -- scan --help > doc/cli/scan.adoc
	cargo run -q -- check --help > doc/cli/check.adoc
	cargo run -q -- load --help > doc/cli/load.adoc

# Generate documentation
doc:
	./scripts/build-doc.sh

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
    #!/bin/sh
    echo "Tagging version v$TAG"
    git tag "v$TAG" -f
    git tag | sort -Vr | head

tag-push:
	#!/bin/sh
	echo "Pushing version v$TAG"
	git push origin "v$TAG"
