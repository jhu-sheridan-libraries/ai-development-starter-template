import pytest

from example_pkg import greet


def test_greet_returns_expected_message():
    assert greet("Sheridan") == "Hello, Sheridan!"


def test_greet_rejects_empty_name():
    with pytest.raises(ValueError):
        greet("")
