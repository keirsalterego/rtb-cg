# rtb-cg

standalone driver to pipe rust-generated llvm-ir directly into the tpde c++ backend.
avoiding the full `codegen_ssa` complexity by handling raw ir translation and ffi bridging manually.

## documentation

primary technical docs for the gsoc proposal review:

* **[architecture.md](docs/architecture.md)**
    high-level design, ssa mapping strategy, and ffi bridge safety details.

* **[ir_spec.md](docs/ir_spec.md)**
    gap analysis between rust's ir output and tpde's capabilities (tracking `overflow` intrinsics, etc).

* **[research_papers/](research_papers/)**
    notes on advanced integration topics, specifically **enzyme** (autodiff) metadata requirements.

## usage

requires `rustc`, `cargo`, and a c++ compiler for the bridge.

```bash
# build the ffi bridge and driver
cargo build

# generate ir from the sample
make gen-ir

# run the driver (mocks the backend connection for now)
cargo run --bin rtb-cli

```

## project structure

* `crates/rtb-core`: pure rust logic for instruction mapping (phi, select, add).
* `crates/rtb-ffi`: `unsafe` bridge to the c++ backend (compiles `bridge.cpp`).
* `crates/rtb-cli`: the main driver executable.

```

```