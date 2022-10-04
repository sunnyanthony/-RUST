# Python
we use `pyo3` to bind python and rust.

## Build Steps
1. run `cargo build`
2. `mv target/debug/librust_pythona.dylib ./rust_python.so`

## Run it !
```python
from rust_python import example1

example1("Anthony")
example_obj = example2(100, "rust!")
assert example_obj.name == "rust!"
```
