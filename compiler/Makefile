# Default target
all: test dot

# Run Rust tests
test:
	cargo test

# Compile all .dot files into .png
dot:
	@echo "Generating PNGs from DOT files..."
	@for file in $(shell find . -name "*.dot"); do \
		echo "Compiling $$file..."; \
		dot -Tpng $$file -o $${file%.dot}.png; \
	done

# Clean PNG output (optional)
clean:
	@echo "Cleaning generated PNG files..."
	@find . -name "*.png" -delete
