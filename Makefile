src_path = src
lib_path = lib
bin_path = bin

libs = -L$(lib_path)

SRC_FILES = $(shell test -e src/ && find src -type f)
EXE_FILES = examples/*.rs

BUILD_LIB = rustc --crate-type=lib --opt-level 3 --out-dir $(lib_path) $(libs)
BUILD_BIN = rustc                  --opt-level 3 --out-dir $(bin_path) $(libs)

all: node

run: node
	bin/node

node:
	mkdir -p $(bin_path)
	$(BUILD_BIN) $(src_path)/node.rs

lib: $(SRC_FILES)
	mkdir -p $(lib_path)
	$(BUILD_LIB) $(src_path)/morph.rs

example: $(EXE_FILES)
	mkdir -p $(bin_path)
	$(BUILD_BIN) example/wot.rs

.PHONY: node