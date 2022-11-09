
.PHONY: install-py
install-py:
	cd chord-chart-py && python3.11 -m venv .venv
	cd chord-chart-py && .venv/bin/pip install maturin
	cd chord-chart-py && .venv/bin/pip install -e .[test]

.PHONY: test-py
test-py:
	cd chord-chart-py && VIRTUAL_ENV=.venv .venv/bin/maturin develop
	cd chord-chart-py && .venv/bin/pytest --color=yes
