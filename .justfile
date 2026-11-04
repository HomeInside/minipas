# Just task runner
#
set dotenv-load := false
just_home := justfile_directory()

# for Windows
#set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

[doc("📋 Show all recipes")]
[group("Help")]
help:
    @just --list

[group("Help")]
default: help

[group("Dev")]
fmt:
	cargo fmt

[group("Dev")]
check:
	clear && cargo fmt && cargo check

[group("Dev")]
clippy:
	cargo clippy --no-deps -- -D clippy::style -D clippy::correctness

[group("Dev")]
fix:
	clear && cargo fmt && cargo check && cargo clippy --no-deps -- -D clippy::style -D clippy::correctness

[group("Dev")]
fix-all:
	clear && cargo fmt && cargo fix --allow-dirty

[group("Dev")]
run:
	clear && cargo fmt && cargo run

[group("Build")]
build:
	clear && cargo fmt && cargo build

[group("Build")]
release:
	clear && cargo fmt && cargo build --release

[group("Dev")]
dist-d:
    ./target/debug/minipas

[group("Dev")]
dist-r:
    ./target/release/minipas

[group("Maintenance")]
[doc('Git check repo')]
gitc:
	git fsck
	git gc --prune=now --aggressive
	git count-objects -vH

[group("Maintenance")]
[doc('clean some files')]
clean:
    rm -fv *.mpc
    rm -fv *.mpp
    rm -fv *.mpa
    rm -fv examples/*.mpc
    rm -fv examples/*.mpp
    rm -fv examples/*.mpa
    rm -fv test/*.mpc
    rm -fv test/*.mpp
    rm -fv test/*.mpa

[group("Maintenance")]
[doc('cleaning the house')]
[windows]
clean-all:clean
    cargo clean && Remove-Item -LiteralPath .\Cargo.lock -Force -Verbose -ErrorAction Ignore && Remove-Item -LiteralPath .\docs\book -Recurse -Force -ErrorAction Ignore

[group("Maintenance")]
[doc('cleaning the house')]
[unix]
clean-all:clean
    cargo clean && rm -fv Cargo.lock && rm -rf ./docs/book

[group("Docs")]
[doc('serve the docs using mdbook')]
docs-serve:
	clear && mdbook serve ./docs

[group("Docs")]
[doc('make website of docs using mdbook')]
docs-build:
	clear && mdbook build ./docs

[group("Docs")]
[doc('clean docs using mdbook')]
docs-clean:
	clear && mdbook clean ./docs

import '.justfile_debug'
import '.justfile_release'
import '.justfile_build'
