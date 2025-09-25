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

test:build
    ./target/debug/minipas run examples/comments.mp
    ./target/debug/minipas run examples/conditionals.mp
    ./target/debug/minipas run examples/constants.mp
    ./target/debug/minipas run examples/data_types.mp
    ./target/debug/minipas run examples/date_time.mp
    ./target/debug/minipas run examples/euclides.mp
    ./target/debug/minipas run examples/fibonacci.mp
    ./target/debug/minipas run examples/for_loop.mp
    ./target/debug/minipas run examples/functions.mp
    ./target/debug/minipas run examples/haversine.mp
    ./target/debug/minipas run examples/hello_world.mp
    ./target/debug/minipas run examples/procedures.mp
    ./target/debug/minipas run examples/program_full.mp
    ./target/debug/minipas run examples/recursion.mp
    ./target/debug/minipas run examples/showcase.mp
    ./target/debug/minipas run examples/std_lib.mp
    ./target/debug/minipas run examples/std_math.mp
    ./target/debug/minipas run examples/strings.mp
    ./target/debug/minipas run examples/sys.mp
    ./target/debug/minipas run examples/variables.mp
    ./target/debug/minipas run examples/while_loop.mp
    ./target/debug/minipas run examples/writeln.mp
    #
    # crear un dump de varios archivos
    #
    ./target/debug/minipas build examples/program_full.mp -o examples/program_full.mpc
    test -f examples/program_full.mpc || (echo "Error: El archivo 'examples/program_full.mpc' no existe." && exit 1)
    #
    ./target/debug/minipas emit examples/program_full.mp
    test -f examples/program_full.mpp || (echo "Error: El archivo 'examples/program_full.mpp' no existe." && exit 1)
    test -f examples/program_full.mpa || (echo "Error: El archivo 'examples/program_full.mpa' no existe." && exit 1)
    #
    ./target/debug/minipas check examples/program_full.mp
    #
    rm -v examples/program_full.mpc
    rm -v examples/program_full.mpp
    rm -v examples/program_full.mpa

test-rel:release
    ./target/release/minipas run examples/comments.mp
    ./target/release/minipas run examples/conditionals.mp
    ./target/release/minipas run examples/constants.mp
    ./target/release/minipas run examples/data_types.mp
    ./target/release/minipas run examples/date_time.mp
    ./target/release/minipas run examples/euclides.mp
    ./target/release/minipas run examples/fibonacci.mp
    ./target/release/minipas run examples/for_loop.mp
    ./target/release/minipas run examples/functions.mp
    ./target/release/minipas run examples/haversine.mp
    ./target/release/minipas run examples/hello_world.mp
    ./target/release/minipas run examples/procedures.mp
    ./target/release/minipas run examples/program_full.mp
    ./target/release/minipas run examples/recursion.mp
    ./target/release/minipas run examples/showcase.mp
    ./target/release/minipas run examples/std_lib.mp
    ./target/release/minipas run examples/std_math.mp
    ./target/release/minipas run examples/strings.mp
    ./target/release/minipas run examples/sys.mp
    ./target/release/minipas run examples/variables.mp
    ./target/release/minipas run examples/while_loop.mp
    ./target/release/minipas run examples/writeln.mp
    #
    # crear un dump de varios archivos
    #
    ./target/debug/minipas build examples/program_full.mp -o examples/program_full.mpc
    test -f examples/program_full.mpc || (echo "Error: El archivo 'examples/program_full.mpc' no existe." && exit 1)
    #
    ./target/debug/minipas emit examples/program_full.mp
    test -f examples/program_full.mpp || (echo "Error: El archivo 'examples/program_full.mpp' no existe." && exit 1)
    test -f examples/program_full.mpa || (echo "Error: El archivo 'examples/program_full.mpa' no existe." && exit 1)
    #
    ./target/debug/minipas check examples/program_full.mp
    #
    rm -v examples/program_full.mpc
    rm -v examples/program_full.mpp
    rm -v examples/program_full.mpa

[doc('Git check repo')]
gitc:
	git fsck
	git gc
