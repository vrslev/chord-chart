import pytest
from chord_chart import transpose_chart
from chord_chart._chord_chart import ValidationError, validate_chart


def test_validate_chart_works():
    assert validate_chart("| C|") == "| C |"


def test_validate_chart_throws():
    with pytest.raises(ValidationError, match="bar line should start with stripe: C"):
        validate_chart("C")


def test_tranpose_chart_works():
    assert transpose_chart("| D |", "D", "Gb") == "| Gb |"


def test_tranpose_chart_throws():
    with pytest.raises(ValidationError, match="bar line should start with stripe: C"):
        transpose_chart("C", "C", "C")
