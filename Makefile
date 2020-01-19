ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

build:
	cd lib/ && cargo build --release
	cp lib/target/release/libhyper.so lib/
	go build -ldflags="-r $(ROOT_DIR)lib" main.go

run: build
	./main