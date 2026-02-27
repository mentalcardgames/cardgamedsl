.PHONY: docs

DOC_OUT = ./docs/architecture/build

docs:
	latexmk -pdf -output-directory=$(DOC_OUT) docs/architecture/architecture.tex