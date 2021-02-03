# https://www.gnu.org/software/make/manual/make.html

### substrate 2.0.0 only works on nightly-2020-10-05
# rustup default nightly-2020-10-05


.PHONY: init
init:
	./scripts/init.sh


.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check --release


.PHONY: check-runtime
check-runtime:
	SKIP_WASM_BUILD=1 cargo check -p runtime


.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --release --all


.PHONY: build
build:
	 cargo build --release


# RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node -lruntime=debug --dev --tmp
.PHONY: debug
debug:
	 RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node -lruntime=debug --dev --tmp


# https://polkadot.js.org/apps
.PHONY: run-debug
run-debug:
	 cargo run -- -lruntime=debug --dev --tmp


#./target/release/node --dev --tmp
.PHONY: run
run:
	 cargo run --release -- --dev --tmp
 # WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo run --release -- --dev --tmp
