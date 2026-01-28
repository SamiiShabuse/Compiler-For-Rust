# Professional Compiler Workspace Structure

Your compiler project has been refactored into a professional multi-crate Rust workspace architecture, similar to production compilers like `rustc`, `clang`, and `kotlin`.

## Directory Structure

```
Compiler-For-Rust/
├── Cargo.toml                          (Workspace root - defines all members)
├── crates/
│   ├── compiler-common/                (Shared types & definitions)
│   │   ├── src/
│   │   │   ├── lib.rs                 (Declares: token, statement)
│   │   │   ├── token.rs               (Token enum with all operators)
│   │   │   └── statement.rs           (Statement and Expr enums)
│   │   └── Cargo.toml
│   │
│   ├── compiler-frontend/              (Lexing & parsing)
│   │   ├── src/
│   │   │   ├── lib.rs                 (Declares & exports: token, tokenizer, parser)
│   │   │   ├── token.rs               (Re-exports Token from compiler-common)
│   │   │   ├── tokenizer.rs           (Tokenizer struct - lexical analysis)
│   │   │   └── parser.rs              (Parser struct - syntax analysis → AST)
│   │   └── Cargo.toml
│   │
│   ├── compiler-ir/                    (Intermediate representation)
│   │   ├── src/
│   │   │   ├── lib.rs                 (Declares & exports: ir, ir_print)
│   │   │   ├── ir.rs                  (IR types: RValue, Primitive, Block, ProgramIR)
│   │   │   └── ir_print.rs            (Pretty-printing IR to human-readable format)
│   │   └── Cargo.toml
│   │
│   ├── compiler-backend/               (Lowering & transformations)
│   │   ├── src/
│   │   │   ├── lib.rs                 (Declares & exports: lower)
│   │   │   └── lower.rs               (lower_main - AST → IR translation)
│   │   └── Cargo.toml
│   │
│   └── compiler-cli/                   (Main binary executable)
│       ├── src/
│       │   ├── main.rs                (Entry point - CLI interface)
│       │   └── lib.rs                 (Empty for now)
│       └── Cargo.toml
│
└── (old src/ folder - can be archived)
```

## Crate Responsibilities

### `compiler-common` 
**Shared types** - No dependencies
- **Token**: Enum of all language tokens (keywords, operators, literals)
- **Statement**: AST statements (print, assignment, control flow)
- **Expr**: Expressions (binary ops, method calls, literals)

### `compiler-frontend`
**Lexing & parsing** - Depends on: `compiler-common`
- **Tokenizer**: Scans input string → produces tokens
- **Parser**: Reads tokens → produces AST (Statements & Exprs)

### `compiler-ir`
**Intermediate representation** - Depends on: `compiler-common`
- **IR Types**: RValue, Primitive operations, Blocks, Control flow
- **Printing**: Formats IR for debugging and output

### `compiler-backend`
**Code generation & transformations** - Depends on: `compiler-common`, `compiler-frontend`, `compiler-ir`
- **Lowering**: Converts high-level AST to low-level IR operations

### `compiler-cli`
**Main binary** - Depends on: all crates
- Entry point with command-line interface
- Orchestrates: tokenize → parse → lower → print

## Dependency Graph

```
compiler-cli
    ↓
    ├→ compiler-frontend  ─→ compiler-common
    ├→ compiler-backend ──┬→ compiler-frontend
    │                     └→ compiler-ir
    └→ compiler-ir ───────→ compiler-common
```

## Build Commands

```bash
# Check code (fast, no linking)
cargo check

# Build all libraries
cargo build --lib

# Build specific crate
cargo build -p compiler-frontend

# The binary build (compiler) may have linker issues due to OneDrive path
# This is a Windows environment issue, not a code issue
# All libraries compile and check successfully
```

## Compilation Flow

When building:
1. `compiler-common` compiles first (no dependencies)
2. `compiler-frontend` compiles next (uses compiler-common)
3. `compiler-ir` compiles (uses compiler-common)
4. `compiler-backend` compiles (uses all above)
5. `compiler-cli` compiles final (uses all above)

Each crate is independently versioned and can be published to crates.io if desired.

## Advantages of This Structure

✅ **Separation of Concerns** - Each crate has a single, clear responsibility
✅ **Modularity** - Easy to test, maintain, and extend each phase
✅ **Reusability** - Others can depend on just the IR or frontend
✅ **Faster Builds** - Only changed crates recompile
✅ **Professional** - Follows industry-standard compiler architecture
✅ **Scalability** - Easy to add new backend passes, optimization phases, etc.

## Next Steps

- Update tests to import from correct crates
- Consider adding more backend passes (optimization, code generation)
- Add support for compilation phases independently (e.g., compile to IR only)
