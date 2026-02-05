
# rtb-cg: architecture & technical specs

documenting the core design decisions and tracking the gap between rust's ir and the tpde backend.

## 1. intermediate representation (rust side)
mapping llvm instructions to a strict rust enum to ensure type safety before hitting the c++ bridge.

```rust
pub enum Instruction {
    Add { dest: String, op1: Operand, op2: Operand },
    Sub { dest: String, op1: Operand, op2: Operand },
    
    // handling ssa control flow merges
    Phi { 
        dest: String, 
        incoming: Vec<(String, Operand)> 
    },
    
    // for ternary logic / conditional moves
    Select {
        dest: String,
        cond: Operand,
        if_true: Operand,
        if_false: Operand,
    },
    Ret(Operand),
}

```

## 2. gap analysis (rust vs tpde)

tracking what instructions map cleanly and where the mismatch is.

| instruction | status | notes |
| --- | --- | --- |
| `add`, `sub` | supported | direct mapping to tpde builder. |
| `phi` | supported | logic handled in `rtb-core`, lowered to c++ array pointers. |
| `select` | supported | maps to ternary logic. |
| `icmp`, `fcmp` | todo | need to map llvm predicate codes to tpde enums. |
| `overflow intrinsics` | unsupported | `llvm.sadd.with.overflow` crashes backend. stripping required. |
| `alloca` | todo | stack frame logic not yet implemented. |

## 3. ffi bridge strategy

avoiding complex c++ bindgen. using a flat, c-style api to keep the bridge robust.

* **data passing:** using `std::ffi::CString` to convert rust `&str` into null-terminated `const char*` for the c++ side.
* **safety:** all external calls are wrapped in `unsafe` blocks. using `cc` in `build.rs` to compile the c++ bridge into a static library.
* **linkage:** `rtb-ffi` exposes safe rust functions (like `emit_phi`) that handle the unsafe pointer casting internally.

## 4. roadmap

* **immediate:** implement the regex-based "intrinsic stripper" in `rtb-cli` to handle `debug` builds.
* **short-term:** map `alloca` and `load/store` to get variables working.
* **strategic:** research `enzyme` metadata integration to support automatic differentiation (mentors mentioned this).
