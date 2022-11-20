# chord-chart

This package provides chord chart validation and transposition functionality.

It is written in Rust and has an accompanying [library](https://github.com/vrslev/chord-chart/tree/main/chord-chart-js) in JavaScript. Used in [vrslev/songbook](https://github.com/vrslev/songbook) — lyrics and chords webapp.

## Example

```py
>>> from chord_chart import ValidationError, transpose_chart, validate_chart

>>> # that's a valid chart: | *chord-with-bass-note* *chord-without-accidental* | *chord* | *(end of the bar, then new bar ->)*
>>> # | *chord-with-accidental-and-symbols* |
>>> validate_chart('| A/E E | E | \n| C#m|')
'| A/E E | E |\n| C#m |'

>>> validate_chart('A/E E\nC#m')  # and that's not a valid one: chords without stripes between lines of bars
Traceback (most recent call last):
...
_chord_chart.ValidationError: bar line should start with stripe: A/E E

>>> transpose_chart('| A/E| E |\n| C#m|', current_key='E', new_key='Db')
'| Gb/Db | Db |\n| Bbm |'

```

## Installation

Just `pip install chord-chart` on Python 3.7 to 3.11.

## Development

- `make install` to clean up and setup dev-env.
- `make test` to build and test the package.
- `make check-types` to run pyright.

Also make sure to install pre-commit hooks (`pre-commit install` from the repository root).
