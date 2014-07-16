src_path = src
lib_path = lib
bin_path = bin
doc_path = doc

nanovg_url	= https://github.com/KevinKelley/nanovg-rs.git
glfw_url	= https://github.com/bjz/glfw-rs.git
gl_url  	= https://github.com/bjz/gl-rs.git

nanovg_path		= lib/nanovg-rs
nanovg_lib_path	= lib/nanovg-rs/lib
glfw_path 		= lib/glfw-rs
glfw_lib_path 	= lib/glfw-rs/lib
gl_path 		= lib/gl-rs
gl_lib_path 	= lib/gl-rs/lib


#libs = -L$(lib_path)
libs = -L$(nanovg_lib_path) -L$(glfw_lib_path) -L$(gl_lib_path)

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
	$(BUILD_LIB) $(src_path)/morphic.rs

example: $(EXE_FILES)
	mkdir -p $(bin_path)
	$(BUILD_BIN) example/wot.rs

get-deps:
	mkdir -p $(lib_path)
	git clone $(nanovg_url) $(nanovg_path)
	git clone $(glfw_url)   $(glfw_path)
	git clone $(gl_url)     $(gl_path)

deps:
	cd $(nanovg_path) && make lib
	make lib -C $(glfw_path)
	#make lib -C $(gl_path)
	cd $(gl_path) && make gen-lib && make -f rust-empty.mk

.PHONY:     	\
	get-deps	\
	deps 		\
	lib