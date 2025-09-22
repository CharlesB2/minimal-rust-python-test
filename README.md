# Minimal Rust Python Test

A minimal test repository for building Python wheels with Rust bindings using GitHub Actions.

This package provides two simple functions:
- `add_numbers(a, b)`: Adds two integers
- `multiply_numbers(a, b)`: Multiplies two floats

## Installation

```bash
pip install minimal-rust-python
```

## Usage

```python
import minimal_rust_python

result = minimal_rust_python.add_numbers(5, 3)  # Returns 8
result = minimal_rust_python.multiply_numbers(2.5, 4.0)  # Returns 10.0
``` 