import pytest
from chord_chart._chord_chart import ValidationError, validate_chart


def test_validate_chart_ok():
    validate_chart("| C |")


def test_validate_chart_throws():
    with pytest.raises(ValidationError, match="bar line should start with stripe: C"):
        validate_chart("C")
