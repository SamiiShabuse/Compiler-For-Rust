# Compiler-For-Rust
This Is The Compiler Im Building For CS441

# Submission Requirements to get a 100

1. Must be able to run this cmd
```bash
./comp p.441 > p.ir
./comp -noopt p.441 > p.ir
```

2. Source to AST conversion
- Compiler must be able to correctly parse all language constructs

3. AST to CFG with correct semantics 
- CFG must correclty implement runtime behavior

4. Tag checks 
- Must generate runtime checks to prevent:
    - Dereferencing integers -> fail NotAPointer
    - Arithmetics on pointers -> fail NotANumber

5. CFG to SSA
- Must convert CFG to SSA form correctly using phi and allowed to used the week-2 ssa inefficient approach

6. One peephole optimization
- Either constnat folding or remove tag checks on this 
- Rules:
    - Optimizaiton must be local to a basic block
    - Must be able to disable with -noopt flag

7. IR output correctness 
- Printed ir must match the spec
- Needs to have:
    data:
    code:
    main:

8. Scripts that demonstrate that my optimizations work
- Submit:
    - 2 .441 programs
    - Optimziation needs to actually be doing something

9. README
    - Which optimzations i implemented
    where in teh code it lives
    any known limiations