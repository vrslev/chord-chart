# Python and JavaScript library for chord chart validation and transposition.

[![pypi](https://img.shields.io/pypi/dm/chord-chart?label=pypi)](https://pypi.org/project/chord-chart)
[![npm](https://img.shields.io/npm/dm/chord-chart-wasm?label=npm)](https://www.npmjs.com/package/chord-chart-wasm)

Validate & format chart in this format:

```
|D G| Bm A|
 | Bm A/C# |D G|Bm A |
```

to this:

```
| D G | Bm A |
| Bm A/C# | D G | Bm A |
```

and traspose the notes to this (from D to H):

```
| H E | G#m F# |
| G#m F#/A# | H E | G#m F# |
```

or any other key.

## Goto

- Python package's [readme →](https://github.com/vrslev/chord-chart/blob/main/chord-chart-py/README.md)
- JavaScript (I mean, TypeScript) package's [readme →](https://github.com/vrslev/chord-chart/blob/main/chord-chart-js/README.md)

## The why

It is used in [vrslev/songbook](https://github.com/vrslev/songbook) — lyrics and chords webapp.
