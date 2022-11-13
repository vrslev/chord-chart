.PHONY: install test

install:
	python3.11 -m venv .venv
	.venv/bin/pip install maturin
	.venv/bin/pip install -e .[test]

test:
	VIRTUAL_ENV=.venv .venv/bin/maturin develop
	.venv/bin/pytest --color=yes
