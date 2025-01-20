# Very Rusty Noisy Qubit

A Rust wrapper for the world's most unreliable quantum computing API. This project demonstrates how to create Rust bindings for a C API, while potentially violating several laws of quantum mechanics.


## What is this?

This is a test project showing how to:
- Use `bindgen` to generate Rust bindings for C API that probably shouldn't exist, obtained who knows from where (submodules for example)
- Create a safe Rust wrapper around unsafe C FFI (because we need more layers of uncertainty)
- Implement tests that might pass (or not, who knows?)
- If tests pass, we can compile and run more tests that will fail (or not, who knows?)


## Building and testing

```bash
cargo build && cargo test
```
