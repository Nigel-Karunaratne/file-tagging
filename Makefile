# Mainly for reference

.PHONY: setup
setup:
	python -m venv .venv
	source .venv/Scripts/activate && pip install maturin pyinstall PyQt5
