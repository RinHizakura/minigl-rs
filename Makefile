all: build

build:
	cargo build

examples: build
	 $(MAKE) -C examples hello
