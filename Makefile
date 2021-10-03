


PYTHON:=venv/bin/python3
PIP:=venv/bin/pip
PYTEST:=venv/bin/pytest
CARGO:=cargo

venv:
	python3 -m venv venv
	${PIP} install wheel

list: venv
	${PIP} list

clean:
	@rm -rf venv
	@rm -rf *.egg-info
	@rm -rf .pytest_cache
	find -iname "*.pyc" -delete
	find -iname "__pycache__" -delete
	${CARGO} clean

build:
	cargo build

install: venv setup.py build
	${PIP} install .

${PYTEST}: venv setup.py
	${PIP} install -e .[dev]

dev-install: ${PYTEST}

status:
	@${PYTHON} --version
	@${CARGO} --version

test: dev-install
	@${PYTEST} tests
