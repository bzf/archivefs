.PHONY: clean libarchivefs

UNAME := $(shell uname)

CFLAGS = -Wall -Wextra -Werror -std=c++11 -fdiagnostics-color=auto \
				 -Iinclude -I/usr/local/include -I/usr/local/opt/libarchive/include
LDFLAGS = -larchive

ifeq ($(UNAME),Darwin)
	LDFLAGS += -losxfuse
	CFLAGS += -D_DARWIN_USE_64_BIT_INODE
endif
ifeq ($(UNAME),Linux)
	LDFLAGS += -lfuse
endif

all: archivefs

archivefs: src/main.cc libarchivefs | create_build_directory
	g++ -D_FILE_OFFSET_BITS=64 -L/usr/local/opt/libarchive/lib -L/usr/local/lib \
		src/main.cc -o archivefs \
		$(LDFLAGS) $(CFLAGS) \
		-L./target/release/ -larchivefs

create_build_directory:
	mkdir -p build/

libarchivefs: src/lib.rs Cargo.toml
	cargo build --release

clean:
	rm -rf build/ archivefs
	cargo clean
