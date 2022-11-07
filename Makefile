.PHONY: test-py
test-py:
	. .venv/bin/activate
	cd chord-chart-py && maturin develop
	pytest
