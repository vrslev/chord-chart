.PHONY: install test

clean:
	rm -rf `find . -name __pycache__`
	rm -rf .pytest_cache
	rm -rf .venv
	rm -rf chord_chart/*.so

install:
	make clean
	python -m venv .venv
	.venv/bin/pip install maturin
	.venv/bin/pip install -e .[test]

test:
	VIRTUAL_ENV=.venv .venv/bin/maturin develop
	.venv/bin/pytest --color=yes --doctest-modules --doctest-glob="*.md"

check-types:
	.venv/bin/pyright
