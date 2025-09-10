# Just task runner

set dotenv-load := false
just_home := justfile_directory()

# help
default: help

# show justfile config file
help:
    @echo
    @echo "The config file is at: {{ just_home }}"
    @echo "projects tasks:"
    @echo
    @echo "- fmt"
    @echo "- fix"
    @echo "- check"
    @echo "- clippy"
    @echo "- serve"
    @echo "- run"
    @echo "- build"
    @echo "- release"
    @echo "- docs"
    @echo "- docs-serve"
    @echo "- opt"
    @echo "- dist"
    @echo "- crud [MODULE_NAME]"
    @echo "- test"
    @echo "- clean"
    @echo

fmt:
	cargo fmt

check:
	clear && cargo fmt && cargo check

clippy:
	clear && cargo fmt && cargo clippy --no-deps

fix:
	clear && cargo fmt && cargo check && cargo clippy --no-deps

fix-all:
	clear && cargo fmt && cargo fix --allow-dirty

run:
	clear && cargo fmt && cargo run

build:
	clear && cargo fmt && cargo build

release:
	clear && cargo fmt && cargo build --release

dist-d:
    ./target/debug/minipas

dist-r:
    ./target/release/minipas

test:
    ./target/debug/minipas ./examples/hello_world.mp

test-rel:
    ./target/release/minipas examples/comments.mp
    ./target/release/minipas examples/conditionals.mp
    ./target/release/minipas examples/constants.mp
    ./target/release/minipas examples/data_types.mp
    ./target/release/minipas examples/date_time.mp
    ./target/release/minipas examples/haversine.mp
    ./target/release/minipas examples/hello_world.mp
    ./target/release/minipas examples/program_full.mp
    ./target/release/minipas examples/std_lib.mp
    ./target/release/minipas examples/std_math.mp
    ./target/release/minipas examples/strings.mp
    ./target/release/minipas examples/sys.mp
    ./target/release/minipas examples/variables.mp
    ./target/release/minipas examples/writeln.mp

[doc('Git check repo')]
gitc:
	git fsck
	git gc
