# Compiler for Rust

A Rust-based compiler construction project for CS441. The repository is organized as a Cargo workspace with separate crates for common compiler data structures, frontend parsing/tokenization, IR, backend lowering, and the command-line interface.

## Project Goals

- Parse source programs into structured compiler representations.
- Convert frontend output into an intermediate representation.
- Lower IR toward executable-style control flow and runtime behavior.
- Support required runtime checks for tagged values.
- Implement and document at least one local optimization pass.
- Keep the workspace modular enough to test individual compiler stages.

## Workspace Layout

```text
.
|-- Cargo.toml
|-- crates/
|   |-- compiler-cli/        # Command-line entry point
|   |-- compiler-common/     # Shared tokens/statements/data types
|   |-- compiler-frontend/   # Tokenizer and parser
|   |-- compiler-ir/         # Intermediate representation and printing
|   `-- compiler-backend/    # Lowering/backend work
|-- docs/
|   |-- WORKSPACE_STRUCTURE.md
|   `-- dev_journal/
`-- tests/
```

## Implemented Areas

- Multi-crate Rust workspace
- Token and statement data structures
- Frontend tokenizer/parser modules
- Intermediate representation modules
- Backend lowering module structure
- CLI crate for compiler execution
- Unit tests for tokens, tokenizer behavior, and library components

## Course Requirements Being Targeted

| Requirement | Notes |
| --- | --- |
| Source to AST conversion | Parse supported language constructs correctly. |
| AST to CFG | Preserve runtime behavior in control-flow form. |
| Runtime tag checks | Detect invalid pointer/integer operations where required. |
| CFG to SSA | Convert control flow into SSA using phi nodes. |
| Peephole optimization | Implement a local optimization that can be disabled with `-noopt`. |
| IR output | Emit the expected `data`, `code`, and `main` sections. |
| Demonstration scripts | Include examples that show the optimization has an observable effect. |

## Expected Usage

The final compiler is expected to support commands in this shape:

```bash
./comp p.441 > p.ir
./comp -noopt p.441 > p.ir
```

## Development

Run tests with:

```bash
cargo test
```

Build the workspace with:

```bash
cargo build
```

## Notes

This repository is both a course project and a systems-learning artifact. The goal is to keep the implementation understandable, tested by stage, and documented honestly as compiler features are completed.