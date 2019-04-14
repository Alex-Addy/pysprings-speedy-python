<!--
$theme: gaia
template: invert
-->

<!-- slides processed and produced using Marp -->

# Faster Python through Rust
# https://github.com/Alex-Addy/pysprings-speedy-python

---

## What is Rust?
"Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety." - rust-lang.org

|||
|:-:|:-:|
|zero-cost abstractions|move semantics|
|guaranteed memory safety|threads without data races|
|trait-based generics|pattern matching|
|type inference|minimal runtime|
|efficient C bindings| |

---

# Python -> Rust Options
- CFFI
- CTypes
- Cython
- [rust-cpython](https://github.com/dgrunwald/rust-cpython)
- [PyO3](https://github.com/PyO3/PyO3)

---

# Steps to faster python
1. Profile
2. Pick better algorithm/data structure
3. Try using `Pypy`
4. If possible use `numpy` or another dedicated library
5. Replace hot code with Rust
6. Rewrite it in Rust

---

# Benchmark Methodology
- Run each thing 10 times
- Each run: generate image 1024x1024
- Each run: complete 50 blending iterations

---

# Benchmark Results
| Impl Type | Time (s) |
|:----------|:--------:|
| Python      | 100.59 |
| Debug Rust   | 48.01 |
| Debug Mixed | 51.01 |
| Pypy        | 11.56 |
| Release Mixed | 1.61 |
| Release Rust | 1.41 |


