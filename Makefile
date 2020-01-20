ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: clear build run

clear:
	rm -rf lib/target
	rm -f lib/libhyper.so 
	rm -f lib/hyper.h

build:
	clear
	cd lib/ && cargo build --release
	cp lib/target/release/libhyper.so lib/
	go build -ldflags="-r $(ROOT_DIR)lib" main.go

run: clear build
	./main
	