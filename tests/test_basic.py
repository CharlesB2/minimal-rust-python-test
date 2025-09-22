import minimal_rust_python


def test_add_numbers():
    result = minimal_rust_python.add_numbers(5, 3)
    assert result == 8


def test_multiply_numbers():
    result = minimal_rust_python.multiply_numbers(2.5, 4.0)
    assert result == 10.0