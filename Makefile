all: build

build:
	cargo build

hello: build
	 $(MAKE) -C examples hello

run: build
	 $(MAKE) -C examples run
