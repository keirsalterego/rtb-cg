.PHONY: gen-ir clean view-ir

# Generates LLVM-IR from the example (Added --crate-type=lib)
gen-ir:
	@mkdir -p build
	rustc --crate-type=lib -C overflow-checks=off --emit=llvm-ir examples/simple_math.rs -o build/simple.ll

# Quick look at the IR in Neovim
view-ir: gen-ir
	nvim build/simple.ll

clean:
	cargo clean
	rm -rf build/
