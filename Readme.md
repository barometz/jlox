This is my attempt to work through Part II of [Crafting
Interpreters](https://craftinginterpreters.com), while also (re?)learning Rust
in the process by writing everything in Rust instead of Java.

To regenerate the [ast enums](src/expr.rs), run `cargo run --bin generate_ast
src`. There's probably a better way to do that through cargo's build.rs thing,
but this is fine.

Extensions:
- [ ] distinguish between integers and floats
- [ ] bitwise negation

Challenges:
- Chapter 4:
    - [x] /* comments */
- Chapter 6
    - [x] conditional ? expressions : work, or at least the parser recognizes them.
    - [x] Appropriate errors for missing left-hand operands to binary operators
